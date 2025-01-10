use serenity::all::{CreateEmbed, InputTextStyle};

pub fn embed() -> CreateEmbed {
    CreateEmbed::new()
        .title("Homebrew Other Submission")
        .field("Your submission has been created.", "You may not interact with the ticket, but you may see it... for now", false)
}

pub enum OtherQuestions {
    Name,
    Link,
    Description,
}

impl super::Questions for OtherQuestions {
    fn get_question(&self) -> &str {
        match &self {
            OtherQuestions::Link => "Link to homebrew",
            OtherQuestions::Name => "What is the name of the homebrew?",
            OtherQuestions::Description => "Give a short description of the homebrew",
        }
    }

    fn get_id(&self) -> &str {
        match &self {
            OtherQuestions::Link => "hb_other_link",
            OtherQuestions::Name => "hb_other_name",
            OtherQuestions::Description => "hb_other_description",
        }    }
    fn required(&    self) -> bool {
        true
    }
    fn style(&self) -> InputTextStyle {
        match self {
            OtherQuestions::Link => InputTextStyle::Short,
            OtherQuestions::Name => InputTextStyle::Short,
            OtherQuestions::Description => InputTextStyle::Short,
        }
    }
}
pub fn get_questions() -> Vec<Box<dyn super::Questions + Send + Sync>> {
    vec![
        Box::from(OtherQuestions::Name),
        Box::from(OtherQuestions::Link),
        Box::from(OtherQuestions::Description),
    ]
}