use crate::data::Data;
use crate::enums::QuestParticipantSelectionMechanism;
use crate::events::send_ephemeral_reply;
use crate::{helpers, Error};
use serenity::all::ComponentInteraction;
use serenity::client::Context;

pub async fn quest_list_all_participants(
    context: &Context,
    interaction: &ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let channel_id = interaction.channel_id.get() as i64;

    let quest_record = sqlx::query!(
        "SELECT bot_message_id, maximum_participant_count, participant_selection_mechanism FROM quest WHERE channel_id = ?",
        channel_id
    )
        .fetch_one(&data.database)
        .await?;

    let selection_mechanism =
        QuestParticipantSelectionMechanism::from_repr(quest_record.participant_selection_mechanism)
            .expect("Should always be valid!");

    let (text, _) = helpers::create_quest_participant_list(
        data,
        channel_id,
        quest_record.maximum_participant_count,
        selection_mechanism,
        false,
    )
    .await?;

    for text in helpers::split_long_messages(text) {
        let _ = send_ephemeral_reply(&interaction, context, &text).await;
    }

    Ok(())
}
