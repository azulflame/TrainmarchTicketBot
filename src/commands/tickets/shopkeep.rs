use serenity::{builder::CreateEmbed, model::prelude::component::InputTextStyle};

use super::Questions;

pub fn embed(embed: &mut CreateEmbed) -> &mut CreateEmbed {
    embed.title("Thanks for your Shopkeep Application!").field(
        "Your application has been created!",
        "A Shopkeep Overseer will review this, and may have further questions for you.",
        false,
    )
}
#[derive(Clone, Copy)]
pub enum ShopkeepQuestions {
    Shop,
    Character,
    Why,
}

impl Questions for ShopkeepQuestions {
    fn get_question(&self) -> &str {
        match &self {
            ShopkeepQuestions::Shop => "Which shop do you want to be a shopkeep for?",
            ShopkeepQuestions::Why => "Why do you want to be a shopkeep?",
            ShopkeepQuestions::Character => "What PC/NPC will you be a shopkeep with?",
        }
    }
    fn get_id(&self) -> &str {
        match &self {
            ShopkeepQuestions::Shop => "shopkeep_modal_shop",
            ShopkeepQuestions::Why => "shopkeep_modal_why",
            ShopkeepQuestions::Character => "shopkeep_modal_character",
        }
    }
    fn required(&self) -> bool {
        match self {
            ShopkeepQuestions::Shop => true,
            ShopkeepQuestions::Character => true,
            ShopkeepQuestions::Why => true,
        }
    }
    fn style(&self) -> InputTextStyle {
        match &self {
            ShopkeepQuestions::Shop => InputTextStyle::Short,
            ShopkeepQuestions::Character => InputTextStyle::Short,
            ShopkeepQuestions::Why => InputTextStyle::Paragraph,
        }
    }
}
pub fn get_questions() -> Vec<Box<dyn super::Questions + Send + Sync>> { vec![
    Box::from(ShopkeepQuestions::Shop),
    Box::from(ShopkeepQuestions::Character),
    Box::from(ShopkeepQuestions::Why),
]}
