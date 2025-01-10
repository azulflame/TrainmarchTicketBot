use serenity::all::InputTextStyle;
use serenity::builder::CreateEmbed;

pub fn embed() -> CreateEmbed {
    CreateEmbed::new()
        .title("Thanks for the DM Application")
        .field("Your application has been created.", "The Head DM will review your answers and may go forward with the process when they have time.", false)
}

#[derive(Clone, Copy)]
pub enum DmQuestions {
    Age,
    Experience,
    ServerTime,
    Vouch,
    Why,
}

impl super::Questions for DmQuestions {
    fn get_question(&self) -> &str {
        match &self {
            DmQuestions::Age => "What is your age?",
            DmQuestions::Experience => "How much experience do you have as a DM?",
            DmQuestions::ServerTime => "How long have you been on the server?",
            DmQuestions::Vouch => "Who on the server can vouch for you?",
            DmQuestions::Why => "Why do you want to be a DM?",
        }
    }
    fn get_id(&self) -> &str {
        match &self {
            DmQuestions::Age => "dm_modal_age",
            DmQuestions::Experience => "dm_modal_experience",
            DmQuestions::ServerTime => "dm_modal_servertime",
            DmQuestions::Vouch => "dm_modal_vouch",
            DmQuestions::Why => "dm_modal_why",
        }
    }
    fn required(&self) -> bool {
        match &self {
            DmQuestions::Age => false,
            DmQuestions::Experience => true,
            DmQuestions::ServerTime => true,
            DmQuestions::Vouch => true,
            DmQuestions::Why => true,
        }
    }
    fn style(&self) -> InputTextStyle {
        match self {
            DmQuestions::Age => InputTextStyle::Short,
            DmQuestions::Experience => InputTextStyle::Paragraph,
            DmQuestions::Vouch => InputTextStyle::Paragraph,
            DmQuestions::ServerTime => InputTextStyle::Paragraph,
            DmQuestions::Why => InputTextStyle::Paragraph,
        }
    }
}

pub fn get_questions() -> Vec<Box<dyn super::Questions + Send + Sync>> {
    vec![
        Box::from(DmQuestions::Age),
        Box::from(DmQuestions::Experience),
        Box::from(DmQuestions::ServerTime),
        Box::from(DmQuestions::Vouch),
        Box::from(DmQuestions::Why),
    ]
}
