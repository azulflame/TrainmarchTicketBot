use serenity::all::{CreateEmbed, InputTextStyle};

pub fn embed() -> CreateEmbed {
    CreateEmbed::new()
        .title("Homebrew Spell Submission")
        .field("Your submission has been created.", "You may not interact with the ticket, but you may see it... for now", false)
}

pub enum SpellQuestions {
    Name,
    Link,
    Level,
    Description,
}

impl super::Questions for SpellQuestions {
    fn get_question(&self) -> &str {
        match &self {
            SpellQuestions::Link => "Link to homebrew spell",
            SpellQuestions::Level => "What is the spell level?",
            SpellQuestions::Name => "What is the spell name?",
            SpellQuestions::Description => "Give a short description of the spell",
        }
    }

    fn get_id(&self) -> &str {
        match &self {
            SpellQuestions::Link => "hb_spell_link",
            SpellQuestions::Level => "hb_spell_rarity",
            SpellQuestions::Name => "hb_spell_name",
            SpellQuestions::Description => "hb_spell_description",
        }    }
    fn required(& self) -> bool {
        true
    }
    fn style(&self) -> InputTextStyle {
        match self {
            SpellQuestions::Link => InputTextStyle::Short,
            SpellQuestions::Level => InputTextStyle::Short,
            SpellQuestions::Name => InputTextStyle::Short,
            SpellQuestions::Description => InputTextStyle::Short,
        }
    }
}
pub fn get_questions() -> Vec<Box<dyn super::Questions + Send + Sync>> {
    vec![
        Box::from(SpellQuestions::Name),
        Box::from(SpellQuestions::Link),
        Box::from(SpellQuestions::Description),
        Box::from(SpellQuestions::Level),
    ]
}