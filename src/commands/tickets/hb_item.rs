use serenity::all::{CreateEmbed, InputTextStyle};

pub fn embed() -> CreateEmbed {
    CreateEmbed::new()
        .title("Homebrew Item Submission")
        .field("Your submission has been created.", "You may not interact with the ticket, but you may see it... for now", false)
}

pub enum ItemQuestions {
    Name,
    Link,
    Rarity,
    Description,
}

impl super::Questions for ItemQuestions {
    fn get_question(&self) -> &str {
        match &self {
            ItemQuestions::Link => "Link to homebrew item",
            ItemQuestions::Rarity => "What is the item rarity?",
            ItemQuestions::Name => "What is the item name?",
            ItemQuestions::Description => "Give a short description of the item",
        }
    }

    fn get_id(&self) -> &str {
        match &self {
            ItemQuestions::Link => "hb_item_link",
            ItemQuestions::Rarity => "hb_item_rarity",
            ItemQuestions::Name => "hb_item_name",
            ItemQuestions::Description => "hb_item_description",
        }    }
    fn required(& self) -> bool {
        true
    }
    fn style(&self) -> InputTextStyle {
        match self {
            ItemQuestions::Link => InputTextStyle::Short,
            ItemQuestions::Rarity => InputTextStyle::Short,
            ItemQuestions::Name => InputTextStyle::Short,
            ItemQuestions::Description => InputTextStyle::Short,
        }
    }
}
pub fn get_questions() -> Vec<Box<dyn super::Questions + Send + Sync>> {
    vec![
        Box::from(ItemQuestions::Name),
        Box::from(ItemQuestions::Link),
        Box::from(ItemQuestions::Description),
        Box::from(ItemQuestions::Rarity),
    ]
}