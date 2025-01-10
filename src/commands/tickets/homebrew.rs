use super::Questions;
use serenity::all::InputTextStyle;
use serenity::builder::CreateEmbed;

pub fn embed() -> CreateEmbed {
    CreateEmbed::new().title("Thanks for your Homebrew Team Application!").field(
        "Your application has been created!",
        "The Head of Homebrew will review this, and may have further questions for you.",
        false,
    )
        .field("Meant to submit an item?", "Items are submitted in <#932470952535658546>. If you meant to submit an item, please let the Head of Homebrew know so they can close it.", false)
}
#[derive(Clone, Copy)]
pub enum HomebrewQuestions {
    Experience,
    Balance,
    Why,
    Time,
}

impl Questions for HomebrewQuestions {
    fn get_question(&self) -> &str {
        match &self {
            HomebrewQuestions::Balance => "How do you determine homebrew balance?",
            HomebrewQuestions::Why => "Why do you want to review homebrew?",
            HomebrewQuestions::Time => "How long have you been on the server?",
            HomebrewQuestions::Experience => "Create a balanced item for each rarity.",
        }
    }
    fn get_id(&self) -> &str {
        match &self {
            HomebrewQuestions::Balance => "homebrew_modal_balance",
            HomebrewQuestions::Why => "homebrew_modal_why",
            HomebrewQuestions::Experience => "homebrew_modal_experience",
            HomebrewQuestions::Time => "homebrew_modal_time",
        }
    }
    fn required(&self) -> bool {
        match self {
            HomebrewQuestions::Balance => true,
            HomebrewQuestions::Experience => true,
            HomebrewQuestions::Why => true,
            HomebrewQuestions::Time => true,
        }
    }
    fn style(&self) -> InputTextStyle {
        match &self {
            HomebrewQuestions::Balance => InputTextStyle::Paragraph,
            HomebrewQuestions::Experience => InputTextStyle::Paragraph,
            HomebrewQuestions::Why => InputTextStyle::Paragraph,
            HomebrewQuestions::Time => InputTextStyle::Paragraph,
        }
    }
}
pub fn get_questions() -> Vec<Box<dyn super::Questions + Send + Sync>> {
    vec![
        Box::from(HomebrewQuestions::Why),
        Box::from(HomebrewQuestions::Experience),
        Box::from(HomebrewQuestions::Balance),
        Box::from(HomebrewQuestions::Time),
    ]
}
