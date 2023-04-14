use serenity::{
    builder::{CreateComponents, CreateEmbed, CreateInteractionResponse},
    model::prelude::interaction::InteractionResponseType,
};

use super::Questions;

pub fn embed(embed: &mut CreateEmbed) -> &mut CreateEmbed {
    embed
    .title("Thanks for your Shopkeep Application!")
    .field("Which Shop?", "Which shop did you want to be a shopkeep for? The Black Market, the Bazaar, or the Trade Market?", false)
    .field("Which Character", "Although it doesn't really matter too much, which character (or NPC) did you want to be a shopkeep", false)
    .field("Someone wants an item when I'm in a quest", "If you're the only shopkeep around and someone wants to buy an item while you're in a quest, you may RP out a brief selling interaction while in the quest. Your DM may not penalize you for this.", false)
}
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
            ShopkeepQuestions::Character => "What character (or NPC) will you be a shopkeep with?",
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
}
const SHOPKEEP_QUESTIONS: [ShopkeepQuestions; 3] = [
    ShopkeepQuestions::Shop,
    ShopkeepQuestions::Character,
    ShopkeepQuestions::Why,
];

pub fn get_modal<'a>(
    z: &'a mut CreateInteractionResponse<'a>,
) -> &'a mut CreateInteractionResponse<'a> {
    z.kind(InteractionResponseType::Modal)
        .interaction_response_data(|f| {
            f.custom_id("dm_modal_submit")
                .components(|c: &mut CreateComponents| {
                    c.create_action_row(|row| super::build_rows(row, &SHOPKEEP_QUESTIONS))
                })
        })
}
