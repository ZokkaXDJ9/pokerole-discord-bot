pub struct GameRule {
    pub name: String,
    pub flavor: Option<String>,
    pub text: String,
    pub example: Option<String>,
}

impl GameRule {
    pub fn get_hardcoded_rules() -> Vec<GameRule> {
        vec![
            GameRule {
                name: String::from("Limit Break"),
                flavor: Some(String::from("By investing an extraordinary amount of effort, some pokemon can surpass their natural limits!")),
                text: String::from("You may spend (2 + Amount of previous Limit Breaks) stat points in order to increase your stats past your species' stat cap. For balancing reasons, you can never have more than 10 points in any particular stat, even by using this mechanic."),
                example: Some(String::from("\
Let's say your max Dexterity is 3. If you want to increase it to 4, you'll need to use two stat points.
Next up, you want to increase your Vitality past its limit. Since you've already used one limit break in the past, this would now cost 3 stat points.
If you stat is already at 10, you cannot limit break it any further."))
            }, GameRule {
                name: String::from("Evolution"),
                flavor: Some(String::from("Sometimes they just grow a little too fast!")),
                text: String::from("You can evolve at any time or even start out as a fully evolved pokemon, but as long as you haven't reached the level required for your evolution yet, you will have to play with the base stats of your unevolved form. Evolution thresholds are Level 3 for second stage, and Level 6 for third stage evolutions. In severe cases where even the second evo has terrible stats, such as for e.g. the Weedle/Kakuna/Beedrill line, you may apply for an exception to be made."),
                example: Some(String::from("Let's say you want to play a Tyranitar. Pupitar is probably not the most fun to play, so you decide to start out fully evolved right from the get go. Until level 3 you will have to play with the base stats of a Larvitar. Once you reach level 3, you can upgrade your base stats to that of a Pupitar, and, finally, once you reach level 6, your base stats may reflect those of a full-powered Tyranitar!"))
            }, GameRule {
                name: String::from("Multi-Target moves"),
                flavor: Some(String::from("\"Watch out, it's the 'Oops, I Did It Everywhere' attack!\"")),
                text: String::from("When using moves targeting *All Foes*, declare the order in which your characters focuses on them. Only the first target can receive a critical hit, and for every successive target hit, reduce the damage die count by 1."),
                example: Some(String::from("You are using Earthquake against three foes! First, you declare the order in which they are to be hit: Ursaring, Absol, Sneasler. Then, you roll accuracy. It's a crit, yay! Roll your regular 8 damage dies against the Ursaring. Critical damage will be applied! Then, roll 7 damage dies against the Lucario. Finally, 6 dies against the Sneasler. Both of those take the regular damage without crit modifiers."))
            }, GameRule {
                name: String::from("Critical Strike"),
                flavor: Some(String::from("Hit 'em right between the legs!")),
                text: String::from("A critical strike occurs when you roll at least three 6 during an accuracy check (You need only 2 for 'High Critical' moves). After the damage reduction from defense, the damage dealt will be increased by 50%, rounded up. If the move applies stat boosts or reductions, those will be increased by 1."),
                example: Some(String::from("You crit and successfully roll 5 damage dies. Your enemy has 2 defense. The final damage dealt is (5 - 2) * 1.5 = 4.5, so 5 damage."))
            }, GameRule {
                name: String::from("Levels and Ranks"),
                flavor: Some(String::from("Now, would you look at this shiny badge?")),
                text: String::from("\
A level up always requires 100 Experience Points. Experience is gained by going on quests and joining in on events, depending on duration and difficulty/danger levels! For each level up, you gain 1 Stat point, which you may freely spend to increase your stats (see Limit Breaking in case you are maxed out.)
You'll gain one combat stat point to allocate for each level up!

Furthermore, Levels decide your guild rank and when you can evolve:
Level 2 => Silver
Level 3 => Evo #1
Level 4 => Gold
Level 6 => Evo #2
Level 8 => Platinum
Level 16 => Diamond

Besides the extra die for all the *+ Rank* accuracy rolls, you'll gain 2 social stat points for each rank up!"),
                example: None
            }, GameRule {
                name: String::from("Wound damage"),
                flavor: Some(String::from("It's just a flesh wound!")),
                text: String::from("Wound damage reduces your maximum HP and needs to be healed over time in base and/or by visiting a doctor. Your only way to heal wounds during a quest is by using Potion Items! If your max HP falls to 0, you lose consciousness and really need some medical assistance!"),
                example: None
            }, GameRule {
                name: String::from("Stat Changes"),
                flavor: Some(String::from("Feeling weak... or... strong? I don't know how to feel.")),
                text: String::from("\
Stat changes done by different moves stack up to a maximum of +/- 3.
Critical stat changes apply/reduce one additional stat point, but not past the limit! If it would boost multiple stats, you got to choose one which will receive the crit bonus.
Defense cannot go below 0, combat stats can't go below 1."),
                example: None
            }, GameRule {
                name: String::from("Alternative Moves (Dodge, Struggle, Help Another)"),
                flavor: Some(String::from("Gotta know the basics if you wanna be an adventurer!")),
                text: String::from("\
Every Character has access to the following moves at any time:
- Struggle
- Help Another
- Dodge\n"),
                example: None
            }, GameRule {
                name: String::from("Accuracy Reduction"),
                flavor: Some(String::from("Got a little bit of dirt in your eye?")),
                text: String::from("Characters afflicted by accuracy reduction (either as a stat change or through a move effect) still roll the regular amount of accuracy dies, but then subtract -X from the amount of successes this roll had."),
                example: Some(String::from("Let's say you want to use Rock Slide. The move has a -1 Accuracy effect. You roll your regular Dex+Rank amount of dies for your accuracy and land two successes. Now we reduce those successes by -1... which leaves us with one success, so the attack still hits, yay!"))
            }, GameRule {
                name: String::from("Defense"),
                flavor: Some(String::from("How much of a beating can you really take?")),
                text: String::from("You Physical Defense is `Vitality / 2`.\nYour Special Defense is `Insight / 2`.nMinimum damage is always 1, unless you resist the attack's type - this is the only case where it gets reduced to 0."),
                example: Some(String::from("With 4 points in Vitality and 5 points in Insight, you'd get 2 physical and 3 special defense!"))
            }, GameRule {
                name: String::from("Orre Character Import"),
                flavor: Some(String::from("Celebi did another woopsiedoodle!")),
                text: String::from("\
While everything that happened over in the Orre Region is considered non-canon since this is taking place in a different timeline and in the far future, you can import your characters from Orre over to this system!
If you where at least Silver Rank, your character may start with Level 2 here.
If you where Gold or higher, your character may start with Level 3.
All of these are optional, of course! And no items or other unlocks can be carried over.
It is fine to start out as an evolved mon here, just check `/rule Evolution` for the details.
Just follow the character creation guide and apply your two level ups afterwards, those will yield you one Combat Stat Point each and promote you to silver, which also yields two extra Social Stat points!"),
                example: None
            }, GameRule {
                name: String::from("Character Slots"),
                flavor: Some(String::from("Sometimes you just need a friend! Sometimes that friend has to be yourself!")),
                text: String::from("\
You get one new character slot ever 5 levels (cumulative across all your characters!).
Using those or creating multiple characters is completely optional, of course.
Also, if you want more or need one earlier, you may request that from our admin team! We'll vote internally on a case-by-case basis to determine if it would work out."),
                example: None
            }
        ]
    }
}
