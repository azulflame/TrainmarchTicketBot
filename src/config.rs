use dotenv::dotenv;
use crate::commands::open::TicketType;

pub enum SecretType {
    HomebrewCategoryId,
    HomebrewSubmission,
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
    HomebrewLogChannel,
}

impl SecretType {
    fn to_string(&self) -> String {
        match self {
            SecretType::HomebrewLogChannel => "HB_LOG_CHANNEL",
            SecretType::HomebrewCategoryId => "HB_CATEGORY_ID",
            SecretType::HomebrewSubmission => "HB_REVIEW_ROLE",
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
            TicketType::HbItem => SecretType::HomebrewSubmission,
            TicketType::HbFeat => SecretType::HomebrewSubmission,
            TicketType::HbSpell => SecretType::HomebrewSubmission,
            TicketType::HbSubclass => SecretType::HomebrewSubmission,
            TicketType::HbOther => SecretType::HomebrewSubmission
        }
    }
}
pub fn load_config() {
    dotenv().ok();
}
pub fn get_config_val(secret_type: SecretType) -> String {
    std::env::var(secret_type.to_string()).expect(format!("secret type {} not set", secret_type.to_string()).as_str())
}
