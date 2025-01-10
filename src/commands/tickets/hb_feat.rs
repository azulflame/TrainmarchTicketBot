use serenity::all::{CreateEmbed, InputTextStyle};

pub fn embed() -> CreateEmbed {
    CreateEmbed::new()
        .title("Homebrew Feat Submission")
        .field("Your submission has been created.", "You may not interact with the ticket, but you may see it... for now", false)
}

pub enum FeatQuestions {
    Name,
    Link,
    Description,
}

impl super::Questions for FeatQuestions {
    fn get_question(&self) -> &str {
        match &self {
            FeatQuestions::Link => "Link to homebrew feat",
            FeatQuestions::Name => "What is the name of the feat?",
            FeatQuestions::Description => "Give a short description of the feat",
        }
    }

    fn get_id(&self) -> &str {
        match &self {
            FeatQuestions::Link => "hb_feat_link",
            FeatQuestions::Name => "hb_feat_name",
            FeatQuestions::Description => "hb_feat_description",
        }    }
    fn required(&    self) -> bool {
        true
    }
    fn style(&self) -> InputTextStyle {
        match self {
            FeatQuestions::Link => InputTextStyle::Short,
            FeatQuestions::Name => InputTextStyle::Short,
            FeatQuestions::Description => InputTextStyle::Short,
        }
    }
}
pub fn get_questions() -> Vec<Box<dyn super::Questions + Send + Sync>> {
    vec![
        Box::from(FeatQuestions::Name),
        Box::from(FeatQuestions::Link),
        Box::from(FeatQuestions::Description),
    ]
}