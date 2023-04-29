use serenity::{builder::CreateEmbed, model::prelude::component::InputTextStyle};

pub fn embed(embed: &mut CreateEmbed) -> &mut CreateEmbed {
    embed.title("Thanks for the Staff Application").field(
        "Your ticket has been created.",
        "We have received your application. We will review it when we have time.",
        false,
    )
}
#[derive(Clone, Copy)]
pub enum StaffQuestions {
    Age,
    Experience,
    ServerTime,
    Why,
}

impl super::Questions for StaffQuestions {
    fn get_question(&self) -> &str {
        match &self {
            StaffQuestions::Age => "What is your age?",
            StaffQuestions::Experience => "How much experience do you have moderating?",
            StaffQuestions::ServerTime => "How long have you been on the server?",
            StaffQuestions::Why => "Why do you want to be a mod?",
        }
    }
    fn get_id(&self) -> &str {
        match &self {
            StaffQuestions::Age => "staff_modal_age",
            StaffQuestions::Experience => "staff_modal_experience",
            StaffQuestions::ServerTime => "staff_modal_servertime",
            StaffQuestions::Why => "staff_modal_why",
        }
    }
    fn required(&self) -> bool {
        match self {
            StaffQuestions::Age => true,
            StaffQuestions::Experience => true,
            StaffQuestions::ServerTime => true,
            StaffQuestions::Why => true,
        }
    }
    fn style(&self) -> InputTextStyle {
        match self {
            StaffQuestions::Age => InputTextStyle::Short,
            StaffQuestions::Experience => InputTextStyle::Paragraph,
            StaffQuestions::ServerTime => InputTextStyle::Paragraph,
            StaffQuestions::Why => InputTextStyle::Paragraph,
        }
    }
}

pub fn get_questions() -> Vec<Box<dyn super::Questions + Send + Sync>> { vec![
    Box::from(StaffQuestions::Age),
    Box::from(StaffQuestions::Experience),
    Box::from(StaffQuestions::ServerTime),
    Box::from(StaffQuestions::Why),
]}
