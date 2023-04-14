use rand::seq::SliceRandom;
use serenity::{
    builder::{CreateComponents, CreateEmbed, CreateInteractionResponse},
    model::prelude::interaction::InteractionResponseType,
};

use super::Questions;
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
pub enum SheetcheckQuestions {
    Age,
    ServerTime,
    Why,
}

impl Questions for SheetcheckQuestions {
    fn get_question(&self) -> &str {
        match &self {
            SheetcheckQuestions::Age => "What is your age?",
            SheetcheckQuestions::ServerTime => "How long have you been on the server?",
            SheetcheckQuestions::Why => "Why do you want to be a Sheetchecker?",
        }
    }
    fn get_id(&self) -> &str {
        match &self {
            SheetcheckQuestions::Age => "dm_modal_age",
            SheetcheckQuestions::ServerTime => "dm_modal_servertime",
            SheetcheckQuestions::Why => "dm_modal_why",
        }
    }
    fn required(&self) -> bool {
        match &self {
            SheetcheckQuestions::Age => false,
            SheetcheckQuestions::ServerTime => true,
            SheetcheckQuestions::Why => true,
        }
    }
}
const SHEETCHECK_QUESTIONS: [SheetcheckQuestions; 3] = [
    SheetcheckQuestions::Age,
    SheetcheckQuestions::ServerTime,
    SheetcheckQuestions::Why,
];

pub fn get_modal<'a>(
    z: &'a mut CreateInteractionResponse<'a>,
) -> &'a mut CreateInteractionResponse<'a> {
    z.kind(InteractionResponseType::Modal)
        .interaction_response_data(|f| {
            f.custom_id("sheetcheck_modal_submit")
                .components(|c: &mut CreateComponents| {
                    c.create_action_row(|r| super::build_rows(r, &SHEETCHECK_QUESTIONS))
                })
        })
}
