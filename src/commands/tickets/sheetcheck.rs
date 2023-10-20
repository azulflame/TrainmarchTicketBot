use rand::seq::SliceRandom;
use serenity::{builder::CreateEmbed, model::prelude::component::InputTextStyle};

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
        .field(
            "Sample Sheets",
            "Please do your best to find the problems with these sample sheets, and the Head of Sheets will get to you when you finish.",
            false,
        )
        .field(
            "Sheet 1",
            GSHEETS
                .choose(&mut rand::thread_rng())
                .as_ref()
                .unwrap()
                .to_string(),
            false,
        )
        .field(
            "Sheet 2",
            DDB.choose(&mut rand::thread_rng())
                .as_ref()
                .unwrap()
                .to_string(),
            false,
        )
        .field(
            "Sheet 3",
            DICECLOUD
                .choose(&mut rand::thread_rng())
                .as_ref()
                .unwrap()
                .to_string(),
            false,
        )
}
#[derive(Clone, Copy)]
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
            SheetcheckQuestions::Age => "sheetcheck_modal_age",
            SheetcheckQuestions::ServerTime => "sheetcheck_modal_servertime",
            SheetcheckQuestions::Why => "sheetcheck_modal_why",
        }
    }
    fn required(&self) -> bool {
        match &self {
            SheetcheckQuestions::Age => false,
            SheetcheckQuestions::ServerTime => true,
            SheetcheckQuestions::Why => true,
        }
    }
    fn style(&self) -> InputTextStyle {
        match &self {
            SheetcheckQuestions::Age => InputTextStyle::Short,
            SheetcheckQuestions::ServerTime => InputTextStyle::Paragraph,
            SheetcheckQuestions::Why => InputTextStyle::Paragraph,
        }
    }
}
pub fn get_questions() -> Vec<Box<dyn super::Questions + Send + Sync>> { vec![
    Box::from(SheetcheckQuestions::Age),
    Box::from(SheetcheckQuestions::ServerTime),
    Box::from(SheetcheckQuestions::Why),
]}
