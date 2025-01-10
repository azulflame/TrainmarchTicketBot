use serenity::builder::CreateEmbed;

use crate::config;

pub fn embed() -> CreateEmbed {
    let role_to_ping = config::get_config_val(config::SecretType::Respec);
    CreateEmbed::new()
    .title("Character Respec")
    .field("Respec Your Character", "Follow the respec guide found in <#821929650753634314> to make your character.", false)
    .field("Meant to Make a Character Instead??", format!("If you meant to make a new character, click on the Character button in <#930713725772648448> and ask a <@&{}> to close this ticket for you.", role_to_ping), false)
    .field("Application Format", "Character Name:\nChanges (list all): \n“Before” Screenshot: \n“After” Screenshot/Link:", false)
}
