use serenity::builder::CreateEmbed;

use crate::config;

pub fn embed() -> CreateEmbed {
    let role_to_ping = config::get_config_val(config::SecretType::Character);
    CreateEmbed::new()
    .title("Character Creation")
    .field("Making Your Character", "Follow the creation guide found in <#821929650753634314> to make your character.", false)
    .field("Meant to Respec?", format!("If you meant to respec an existing character, click on the Respec button in <#930713725772648448> and ask a <@&{}> to close this ticket for you.", role_to_ping), false)
    .field("Application Format", "Character Name: \nWanderer/Native: \nCharacter [Sub]Class: \nCharacter Level: \nCharacter Race: \nCharacter Background: \nStat Creation Method: \nOptional Feat: \nOptional Common Magic Item: \nIf applicable, retired stamps/gold: \nSheet Link: ", false)
}
