use serenity::builder::CreateEmbed;

pub fn embed(embed: &mut CreateEmbed) -> &mut CreateEmbed {
    embed
    .title("Thanks for your Shopkeep Application!")
    .field("Which Shop?", "Which shop did you want to be a shopkeep for? The Black Market, the Bazaar, or the Trade Market?", false)
    .field("Which Character", "Although it doesn't really matter too much, which character (or NPC) did you want to be a shopkeep", false)
    .field("Someone wants an item when I'm in a quest", "If you're the only shopkeep around and someone wants to buy an item while you're in a quest, you may RP out a brief selling interaction while in the quest. Your DM may not penalize you for this.", false)
}
