use std::{collections::BTreeMap, sync::Once};

use shuttle_secrets::SecretStore;

use crate::commands::open::TicketType;

static mut CONFIG: Option<SecretStore> = None;
static INIT_CONFIG: Once = Once::new();

pub enum SecretType {
    DiscordToken,
    CategoryId,
    LogChannel,
    Sheetcheck,
    Character,
    Homebrew,
    BotRole,
    GuildId,
    Respec,
    Staff,
    Lore,
    Dm,
}

impl SecretType {
    fn to_string(&self) -> String {
        match self {
            SecretType::DiscordToken => "DISCORD_TOKEN",
            SecretType::CategoryId => "CATEGORY_ID",
            SecretType::LogChannel => "LOG_CHANNEL",
            SecretType::Sheetcheck => "SHEETCHECK",
            SecretType::Character => "CHARACTER",
            SecretType::Homebrew => "HOMEBREW",
            SecretType::GuildId => "GUILD_ID",
            SecretType::BotRole => "BOT_ROLE",
            SecretType::Respec => "RESPEC",
            SecretType::Staff => "STAFF",
            SecretType::Lore => "LORE",
            SecretType::Dm => "DM",
        }
        .to_string()
    }
}
impl From<TicketType> for SecretType {
    fn from(tt: TicketType) -> SecretType {
        match tt {
            TicketType::Character => SecretType::Character,
            TicketType::Respec => SecretType::Respec,
            TicketType::Dm => SecretType::Dm,
            TicketType::Homebrew => SecretType::Homebrew,
            TicketType::Sheetcheck => SecretType::Sheetcheck,
            TicketType::Staff => SecretType::Staff,
            TicketType::Lore => SecretType::Lore,
        }
    }
}

pub fn load_config(ss: SecretStore) {
    unsafe {
        INIT_CONFIG.call_once(|| {
            CONFIG = Some(ss.clone());
        })
    }
}

pub fn get_config_val(secret_type: SecretType) -> String {
    unsafe {
        INIT_CONFIG.call_once(|| {
            CONFIG = Some(SecretStore::new(BTreeMap::new()));
        });
        CONFIG
            .as_ref()
            .unwrap()
            .get(secret_type.to_string().as_str())
            .unwrap()
    }
}
