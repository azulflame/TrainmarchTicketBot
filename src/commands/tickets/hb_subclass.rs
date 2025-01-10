use serenity::all::{CreateEmbed, InputTextStyle};
use serenity::futures::task::SpawnExt;

pub fn embed() -> CreateEmbed {
    CreateEmbed::new()
        .title("Homebrew Subclass Submission")
        .field("Your submission has been created.", "You may not interact with the ticket, but you may see it... for now", false)
}

pub enum SubclassQuestions {
    Name,
    Link,
    Class,
    Description,
}

impl super::Questions for SubclassQuestions {
    fn get_question(&self) -> &str {
        match &self {
            SubclassQuestions::Link => "Link to homebrew subclass",
            SubclassQuestions::Class => "What class is this for?",
            SubclassQuestions::Name => "What is the subclass name?",
            SubclassQuestions::Description => "Give a short description of the subclass",
        }
    }

    fn get_id(&self) -> &str {
        match &self {
            SubclassQuestions::Link => "hb_subclass_link",
            SubclassQuestions::Class => "hb_subclass_class",
            SubclassQuestions::Name => "hb_subclass_name",
            SubclassQuestions::Description => "hb_subclass_description",
        }    }
    fn required(&    self) -> bool {
        true
    }
    fn style(&self) -> InputTextStyle {
        match self {
            SubclassQuestions::Link => InputTextStyle::Short,
            SubclassQuestions::Class => InputTextStyle::Short,
            SubclassQuestions::Name => InputTextStyle::Short,
            SubclassQuestions::Description => InputTextStyle::Short,
        }
    }
}
pub fn get_questions() -> Vec<Box<dyn super::Questions + Send + Sync>> {
    vec![
        Box::from(SubclassQuestions::Name),
        Box::from(SubclassQuestions::Link),
        Box::from(SubclassQuestions::Description),
        Box::from(SubclassQuestions::Class),
    ]
}