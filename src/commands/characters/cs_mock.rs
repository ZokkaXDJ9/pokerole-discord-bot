use crate::commands::{Context, Error};
use crate::emoji;
use poise::CreateReply;
use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, ReactionType};

/// WIP
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn cs_mock_1(ctx: Context<'_>) -> Result<(), Error> {
    let message_content = "*(This is a mock in order to gather ideas and feedback, nothing like this has been implemented yet.)*

## <:badge_gold:1119185974149251092> Pikatest <a:pikachu_female_animated:1194179809899712574> 
Level 5 `(42 / 100)`
1337 <:poke_coin:1120237132200546304>
### Stats <:type_electric:1118594871272415243>
```
HP: 8
Will: 4

Strength:  2 |⬤⬤⭘⭘
Dexterity: 2 |⬤⬤⭘⭘⭘
Vitality:  1 |⬤⭘⭘
Special:   2 |⬤⬤⭘⭘
Insight:   2 |⬤⬤⭘⭘

Defense: 1
Special Defense: 1

Tough:  1 |⬤⭘⭘⭘⭘
Cool:   1 |⬤⭘⭘⭘⭘
Beauty: 1 |⬤⭘⭘⭘⭘
Cute:   1 |⬤⭘⭘⭘⭘
Clever: 1 |⬤⭘⭘⭘⭘
```
### Abilities
- Static
- Lightning Rod (Hidden)
### Statistics
🎒 Backpack Slots: 6

🏆 Completed Quests: 4
🤺 Total Sparring Sessions: 3
🎫 Given tours: 1
";

    ctx.send(
        CreateReply::default()
            .content(message_content)
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new("ignore_distribute-stats")
                    .label("9 Remaining Stat Points")
                    .style(ButtonStyle::Primary),
                CreateButton::new("ignore_distribute-socials")
                    .label("8 Remaining Social Points")
                    .style(ButtonStyle::Primary),
            ])]),
    )
    .await?;

    Ok(())
}

/// WIP
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn cs_mock_2(ctx: Context<'_>) -> Result<(), Error> {
    let message_content = "*(This is a mock in order to gather ideas and feedback and see how the buttons would look like on different devices, nothing like this has been implemented yet.)*

## <:badge_gold:1119185974149251092> Pikatest <a:pikachu_female_animated:1194179809899712574> 
Press the buttons below to apply your stat points. Once done, don't forget to press the apply button. You don't have to spend all your points at once.
```
HP: 8
Will: 4

Strength:  2 |⬤⬤⭘⭘
Dexterity: 2 |⬤⬤⭘⭘⭘
Vitality:  1 |⬤⭘⭘
Special:   2 |⬤⬤⭘⭘
Insight:   2 |⬤⬤⭘⭘

Defense: 2
Special Defense: 2
```
";

    ctx.send(
        CreateReply::default()
            .content(message_content)
            .components(vec![
                CreateActionRow::Buttons(vec![
                    CreateButton::new("ignore_stat-add-str")
                        .label("+STR")
                        .style(ButtonStyle::Success),
                    CreateButton::new("ignore_stat-add-dex")
                        .label("+DEX")
                        .style(ButtonStyle::Success),
                    CreateButton::new("ignore_stat-add-vit")
                        .label("+VIT")
                        .style(ButtonStyle::Success),
                    CreateButton::new("ignore_stat-add-spe")
                        .label("+SPE")
                        .style(ButtonStyle::Success),
                    CreateButton::new("ignore_stat-add-ins")
                        .label("+INS")
                        .style(ButtonStyle::Success),
                ]),
                CreateActionRow::Buttons(vec![
                    CreateButton::new("ignore_stat-subtract-str")
                        .label("-STR")
                        .style(ButtonStyle::Danger),
                    CreateButton::new("ignore_stat-subtract-dex")
                        .label("-DEX")
                        .style(ButtonStyle::Danger),
                    CreateButton::new("ignore_stat-subtract-vit")
                        .label("-VIT")
                        .style(ButtonStyle::Danger),
                    CreateButton::new("ignore_stat-subtract-spe")
                        .label("-SPE")
                        .style(ButtonStyle::Danger),
                    CreateButton::new("ignore_stat-subtract-ins")
                        .label("-INS")
                        .style(ButtonStyle::Danger),
                ]),
                CreateActionRow::Buttons(vec![
                    CreateButton::new("ignore_stat-apply")
                        .label("Apply")
                        .emoji(ReactionType::Unicode(emoji::UNICODE_CHECK_MARK.to_string()))
                        .style(ButtonStyle::Primary),
                    CreateButton::new("ignore_stat-cancel")
                        .label("Cancel")
                        .emoji(ReactionType::Unicode(emoji::UNICODE_CROSS_MARK.to_string()))
                        .style(ButtonStyle::Secondary),
                ]),
            ]),
    )
    .await?;

    Ok(())
}
