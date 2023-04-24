use serenity::{builder::CreateEmbed, model::prelude::component::InputTextStyle};

pub fn embed(embed: &mut CreateEmbed) -> &mut CreateEmbed {
    embed
        .title("Thanks for the Lore Team Application")
        .field("Your application has been created.", "The Lore Team will review your answers and may go forward with the process when they have time.", false)
        .field("Short Story", "In this ticket, write me a short story about a chicken that crossed the road. Minimum 5 sentences.", false)
}

#[derive(Clone, Copy)]
pub enum LoreQuestions {
    Age,
    ServerTime,
    Vouch,
    Why,
}

impl super::Questions for LoreQuestions {
    fn get_question(&self) -> &str {
        match &self {
            LoreQuestions::Age => "What is your age?",
            LoreQuestions::ServerTime => "How long have you been on the server?",
            LoreQuestions::Vouch => "Who on the server can vouch for you?",
            LoreQuestions::Why => "Why do you want to be on the Lore Team?",
        }
    }
    fn get_id(&self) -> &str {
        match &self {
            LoreQuestions::Age => "lore_modal_age",
            LoreQuestions::ServerTime => "lore_modal_servertime",
            LoreQuestions::Vouch => "lore_modal_vouch",
            LoreQuestions::Why => "lore_modal_why",
        }
    }
    fn required(&self) -> bool {
        match &self {
            LoreQuestions::Age => false,
            LoreQuestions::ServerTime => true,
            LoreQuestions::Vouch => true,
            LoreQuestions::Why => true,
        }
    }
    fn style(&self) -> InputTextStyle {
        match self {
            LoreQuestions::Age => InputTextStyle::Short,
            LoreQuestions::Vouch => InputTextStyle::Paragraph,
            LoreQuestions::ServerTime => InputTextStyle::Paragraph,
            LoreQuestions::Why => InputTextStyle::Paragraph,
        }
    }
}
pub const LORE_QUESTIONS: [LoreQuestions; 4] = [
    LoreQuestions::Age,
    LoreQuestions::ServerTime,
    LoreQuestions::Vouch,
    LoreQuestions::Why,
];
