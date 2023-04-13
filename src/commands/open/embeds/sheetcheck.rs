use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
const GSHEETS: &[&str] = &["https://docs.google.com/spreadsheets/d/1Yj-nmFw86gp2zh3cLqvJ5SEoBZ_hR7UJsSol2s2NdFk/edit?usp=drivesdk",
"https://docs.google.com/spreadsheets/d/14MOQ-dtfVn5ua9z5ypfe0T1TnB-u-eitONIDwWhfW-c/edit?usp=drivesdk",
"https://docs.google.com/spreadsheets/d/1zpHxzrpfX3wUbNr06fOkVyLbL6BAqX5lH-wDQAqLQfQ/edit?usp=drivesdk"];
const DDB: &[&str] = &[
    "https://www.dndbeyond.com/characters/49606322",
    "https://www.dndbeyond.com/characters/81477563",
    "https://www.dndbeyond.com/characters/82923838",
];
const DICECLOUD: &[&str] = &[
    "https://dicecloud.com/character/HSuBGW4zfLFAuWMcC/Gooble",
    "https://v1.dicecloud.com/character/t7Ex4jZ6k5ty7iZ3r/-",
    "https://v1.dicecloud.com/character/ZH5vi3n42DnzQP95R/-",
];

pub fn embed(embed: &mut CreateEmbed) -> &mut CreateEmbed {
    embed
    .title("Thanks for the Sheetchecker Application")
    .field("A Few Questions", "What is your age (optional)?\nHow much time have you spent on the server?\nWhat is your incentive for sheet checking?", false)
    .field("", "Please answer the questons above, then we will have you check a few sheets.", false)
    .field("Sheet 1", GSHEETS.choose(&mut rand::thread_rng()).as_ref().unwrap().to_string(), false)
    .field("Sheet 2", DDB.choose(&mut rand::thread_rng()).as_ref().unwrap().to_string(), false)
    .field("Sheet 3", DICECLOUD.choose(&mut rand::thread_rng()).as_ref().unwrap().to_string(), false)
}
