use serenity::{builder::CreateEmbed, model::prelude::component::InputTextStyle};

pub fn embed(embed: &mut CreateEmbed) -> &mut CreateEmbed {
    embed
        .title("Thanks for the Lore Team Application")
        .field("Your application has been created.", "The Lore Team will review your answers and may go forward with the process when they have time.", false)
}

pub enum LoreQuestions {
    Age,
    ServerTime,
    Vouch,
    Why,
    Story,
}

impl super::Questions for LoreQuestions {
    fn get_question(&self) -> &str {
        match &self {
            LoreQuestions::Age => "What is your age?",
            LoreQuestions::ServerTime => "How long have you been on the server?",
            LoreQuestions::Vouch => "Who on the server can vouch for you?",
            LoreQuestions::Why => "Why do you want to be on the Lore Team?",
            LoreQuestions::Story => "Tell me why the chicken crossed the road.",
        }
    }
    fn get_id(&self) -> &str {
        match &self {
            LoreQuestions::Age => "dm_modal_age",
            LoreQuestions::ServerTime => "dm_modal_servertime",
            LoreQuestions::Vouch => "dm_modal_vouch",
            LoreQuestions::Why => "dm_modal_why",
            LoreQuestions::Story => "lore_modal_story",
        }
    }
    fn required(&self) -> bool {
        match &self {
            LoreQuestions::Age => false,
            LoreQuestions::Story => true,
            LoreQuestions::ServerTime => true,
            LoreQuestions::Vouch => true,
            LoreQuestions::Why => true,
        }
    }
    fn style(&self) -> InputTextStyle {
        match self {
            LoreQuestions::Age => InputTextStyle::Short,
            LoreQuestions::Story => InputTextStyle::Paragraph,
            LoreQuestions::Vouch => InputTextStyle::Paragraph,
            LoreQuestions::ServerTime => InputTextStyle::Paragraph,
            LoreQuestions::Why => InputTextStyle::Paragraph,
        }
    }
}
pub const LORE_QUESTIONS: [LoreQuestions; 5] = [
    LoreQuestions::Age,
    LoreQuestions::ServerTime,
    LoreQuestions::Vouch,
    LoreQuestions::Why,
    LoreQuestions::Story,
];
