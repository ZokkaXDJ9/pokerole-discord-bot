use std::sync::Arc;

use serenity::all::{
    ComponentInteraction, ComponentInteractionDataKind, CreateActionRow, CreateAllowedMentions,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, EditMessage,
    FullEvent, GuildId, GuildMemberUpdateEvent, HttpError, Interaction, Member, Message, MessageId,
    User,
};
use serenity::client::Context;
use serenity::model::id::ChannelId;
use sqlx::{Pool, Sqlite};

use crate::data::Data;
use crate::game_data::GameData;
use crate::{discord_error_codes, helpers, Error};

mod backups;
mod button_interaction;
mod character_stat_edit;
mod monthly_reset;
mod quests;
mod role_reaction;
mod select_menu_interaction;
mod weekly_reset;

type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, Error>;

pub async fn handle_events<'a>(
    context: &'a Context,
    event: &FullEvent,
    framework: FrameworkContext<'a>,
) -> Result<(), Error> {
    match event {
        FullEvent::InteractionCreate { interaction } => {
            handle_interaction(context, framework, interaction).await
        }
        FullEvent::ReactionAdd { add_reaction } => {
            role_reaction::handle_reaction_add(context, framework, add_reaction).await
        }
        FullEvent::ReactionRemove { removed_reaction } => {
            role_reaction::handle_reaction_remove(context, framework, removed_reaction).await
        }
        FullEvent::GuildMemberRemoval { guild_id, user, .. } => {
            handle_guild_member_removal(context, framework.user_data, guild_id, user).await
        }
        FullEvent::GuildMemberUpdate {
            old_if_available,
            new,
            event,
        } => {
            if let Some(new) = new {
                handle_guild_member_update(context, &framework, new, event).await
            } else {
                let _ = helpers::ERROR_LOG_CHANNEL
                    .send_message(&context, CreateMessage::new().content(
                        format!("Encountered a weird edge case in GuildMemberUpdate.\n old: {:?}\n new: {:?}, event: {:?}",
                                old_if_available, new, event)))
                    .await;

                Ok(())
            }
        }
        FullEvent::GuildMemberAddition { new_member } => {
            // TODO: Send greeting, add default roles
            Ok(())
        }
        FullEvent::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id,
        } => {
            // TODO: Maybe log message deletion
            Ok(())
        }
        FullEvent::MessageUpdate {
            old_if_available,
            new,
            event,
        } => {
            // TODO: Maybe log message edit
            Ok(())
        }
        FullEvent::Ready { .. } => {
            // TODO: Could use the data inside this event to lazily count how many discord servers are using the bot.
            backups::start_backup_thread(context, framework.user_data).await;
            weekly_reset::start_weekly_reset_thread(context, framework.user_data).await;
            monthly_reset::start_monthly_reset_thread(context, framework.user_data).await;
            Ok(())
        }
        _ => Ok(()),
    }
}

async fn handle_guild_member_update(
    ctx: &Context,
    framework: &FrameworkContext<'_>,
    new: &Member,
    event: &GuildMemberUpdateEvent,
) -> Result<(), Error> {
    let user_id = new.user.id.get() as i64;
    let guild_id = new.guild_id.get() as i64;

    let nickname = match &new.nick {
        None => &new.user.name,
        Some(nick) => nick,
    };

    let result = sqlx::query!(
        "
INSERT INTO user_in_guild (name, user_id, guild_id) VALUES (?, ?, ?) 
ON CONFLICT(user_id, guild_id) DO UPDATE SET name = ?",
        nickname,
        user_id,
        guild_id,
        nickname,
    )
    .execute(&framework.user_data.database)
    .await;

    framework
        .user_data
        .cache
        .reset(&framework.user_data.database)
        .await;

    Ok(())
}

async fn handle_guild_member_removal(
    context: &Context,
    data: &Data,
    guild_id: &GuildId,
    user: &User,
) -> Result<(), Error> {
    // TODO: Should be a Database setting instead of being hardcoded.
    let channel_id: u64;
    let user_name = &user.name;
    let user_id = user.id.get() as i64;
    let guild_id = guild_id.get() as i64;
    if guild_id == 1113123066059436093 {
        // Explorers of the Sea
        channel_id = 1113127675586941140;
    } else if guild_id == 1115690620342763645 {
        // Test Server
        channel_id = 1120344272571486309;
    } else {
        return Ok(());
    }

    let character_names = sqlx::query!(
        "SELECT name FROM character WHERE user_id = ? AND guild_id = ?",
        user_id,
        guild_id
    )
    .fetch_all(&data.database)
    .await;

    let names;
    if let Ok(character_names) = character_names {
        if character_names.is_empty() {
            names = String::from("didn't find any characters for them in the database");
        } else {
            names = character_names
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<String>>()
                .join(", ");
        }
    } else {
        names = String::from("failed to check database for matching character names...?");
    }

    let channel = ChannelId::from(channel_id);
    channel
        .send_message(
            context,
            CreateMessage::new()
                .content(&format!(
                    "{}/{} ({}) has left the server.",
                    user_name, user, names
                ))
                .allowed_mentions(CreateAllowedMentions::default().empty_users()),
        )
        .await?;

    Ok(())
}

async fn handle_interaction(
    context: &Context,
    framework: FrameworkContext<'_>,
    interaction: &Interaction,
) -> Result<(), Error> {
    match interaction {
        Interaction::Component(component) => {
            handle_message_component_interaction(context, framework, component).await
        }
        _ => Ok(()),
    }
}

async fn handle_message_component_interaction(
    context: &Context,
    framework: FrameworkContext<'_>,
    interaction: &ComponentInteraction,
) -> Result<(), Error> {
    match &interaction.data.kind {
        ComponentInteractionDataKind::Button => {
            button_interaction::handle_button_interaction(context, framework, &interaction).await?
        }
        ComponentInteractionDataKind::StringSelect { .. } => {
            select_menu_interaction::handle_select_menu_interaction(
                context,
                framework,
                &interaction,
            )
            .await?
        }
        ComponentInteractionDataKind::UserSelect { .. } => {}
        ComponentInteractionDataKind::RoleSelect { .. } => {}
        ComponentInteractionDataKind::MentionableSelect { .. } => {}
        ComponentInteractionDataKind::ChannelSelect { .. } => {}
        ComponentInteractionDataKind::Unknown(_) => {}
    }

    Ok(())
}

fn parse_interaction_command(custom_id: &str) -> (&str, Vec<&str>) {
    let mut split = custom_id.split('_');
    let command = split.next();
    let args: Vec<&str> = split.collect();

    (
        command.expect("Commands should never be empty at this point!"),
        args,
    )
}

async fn send_ephemeral_reply(
    interaction: &&ComponentInteraction,
    context: &Context,
    content: &str,
) -> Result<(), Error> {
    interaction
        .create_response(
            context,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .ephemeral(true)
                    .content(content),
            ),
        )
        .await?;
    Ok(())
}

async fn send_error(
    interaction: &&ComponentInteraction,
    context: &Context,
    content: &str,
) -> Result<(), Error> {
    send_ephemeral_reply(interaction, context, content).await
}

async fn send_error_to_log_channel(ctx: &Context, message: impl Into<String>) {
    let _ = helpers::ERROR_LOG_CHANNEL
        .send_message(ctx, CreateMessage::new().content(message))
        .await;
}

async fn update_character_post<'a>(
    ctx: &Context,
    database: &Pool<Sqlite>,
    game_data: &Arc<GameData>,
    id: i64,
) {
    if let Some(result) =
        crate::commands::characters::build_character_string(database, game_data, id).await
    {
        let message = ctx
            .http
            .get_message(
                ChannelId::from(result.stat_channel_id as u64),
                MessageId::from(result.stat_message_id as u64),
            )
            .await;
        if let Ok(mut message) = message {
            if let Err(e) = message
                .edit(
                    ctx,
                    EditMessage::new()
                        .content(&result.message)
                        .components(result.components.clone()),
                )
                .await
            {
                handle_error_during_message_edit(
                    ctx,
                    e,
                    message,
                    result.message,
                    Some(result.components),
                    result.name,
                )
                .await;
            }
        }
    }
}

async fn handle_error_during_message_edit(
    ctx: &Context,
    e: serenity::Error,
    mut message_to_edit: Message,
    updated_message_content: impl Into<String>,
    components: Option<Vec<CreateActionRow>>,
    name: impl Into<String>,
) {
    if let serenity::Error::Http(HttpError::UnsuccessfulRequest(e)) = &e {
        if e.error.code == discord_error_codes::ARCHIVED_THREAD {
            if let Ok(channel) = message_to_edit.channel(ctx).await {
                if let Some(channel) = channel.guild() {
                    if let Ok(response) = channel
                        .say(ctx, "This thread was (probably) automagically archived, and I'm sending this message to reopen it so I can update some values. This message should be deleted right away, sorry if it pinged you!").await
                    {
                        let _ = response.delete(ctx).await;
                        let mut edit_message = EditMessage::new().content(updated_message_content);
                        if let Some(components) = components {
                            edit_message = edit_message.components(components);
                        }

                        if let Err(e) = message_to_edit.edit(ctx, edit_message).await {
                            let _ = helpers::ERROR_LOG_CHANNEL.send_message(ctx, CreateMessage::new().content(format!(
                                "**Failed to update the stat message for {}!**.\nThe change has been tracked, but whilst updating the message some error occurred:\n```{:?}```\n",
                                name.into(),
                                e,
                            ))).await;
                        }

                        return;
                    }
                }
            }
        }
    }

    let _ = helpers::ERROR_LOG_CHANNEL.send_message(ctx, CreateMessage::new().content(format!(
        "Some very random error occurred when updating the stat message for {}.\n**The requested change has been applied, but it isn't shown in the message there right now.**\n Error:\n```{:?}```",
        name.into(), e)
    )).await;
}
