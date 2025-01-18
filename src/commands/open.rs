use std::str::FromStr;

use super::tickets::{
    character, dm, get_question_from_id, homebrew, lore, respec, send_modal, sheetcheck, staff,
    hb_item, hb_feat, hb_other, hb_spell, hb_subclass
};
use crate::config::{self, SecretType};
use serenity::all::{ActionRowComponent, ComponentInteraction, CreateMessage, ModalInteraction};
use serenity::builder::CreateChannel;
use serenity::{
    builder::CreateEmbed,
    model::{prelude::*, Permissions},
    prelude::Context,
};
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Question {
    title: String,
    answer: String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TicketType {
    Character,
    Respec,
    Dm,
    Sheetcheck,
    Staff,
    Lore,
    Homebrew,
    HbItem,
    HbFeat,
    HbSpell,
    HbSubclass,
    HbOther,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TicketInformation {
    ticket_type: TicketType,
    author_name: String,
    author: UserId,
    author_id: u64,
    guild_id: u64,
    guild: GuildId,
    questions: Vec<Question>,
}

impl FromStr for TicketType {
    type Err = ();
    fn from_str(input: &str) -> Result<TicketType, ()> {
        println!("{}", &input);
        match input {
            "create_character_ticket" => Ok(TicketType::Character),
            "create_respec_ticket" => Ok(TicketType::Respec),
            "create_dm_ticket" => Ok(TicketType::Dm),
            "create_sheetcheck_ticket" => Ok(TicketType::Sheetcheck),
            "create_staff_ticket" => Ok(TicketType::Staff),
            "create_lore_ticket" => Ok(TicketType::Lore),
            "create_homebrew_ticket" => Ok(TicketType::Homebrew),
            "lore_ticket_modal" => Ok(TicketType::Lore),
            "dm_ticket_modal" => Ok(TicketType::Dm),
            "sheetcheck_ticket_modal" => Ok(TicketType::Sheetcheck),
            "staff_ticket_modal" => Ok(TicketType::Staff),
            "homebrew_ticket_modal" => Ok(TicketType::Homebrew),
            "hb_item_modal" => Ok(TicketType::HbItem),
            "hb_feat_modal" => Ok(TicketType::HbFeat),
            "hb_spell_modal" => Ok(TicketType::HbSpell),
            "hb_other_modal" => Ok(TicketType::HbOther),
            "hb_subclass_modal" => Ok(TicketType::HbSubclass),
            "create_homebrew_item" => Ok(TicketType::HbItem),
            "create_homebrew_feat" => Ok(TicketType::HbFeat),
            "create_homebrew_spell" => Ok(TicketType::HbSpell),
            "create_homebrew_subclass" => Ok(TicketType::HbSubclass),
            "create_homebrew_other" => Ok(TicketType::HbOther),
            _ => Err(()),
        }
    }
}

impl TicketType {
    fn to_string(&self) -> String {
        match self {
            TicketType::Character => "character",
            TicketType::Respec => "respec",
            TicketType::Dm => "dm",
            TicketType::Sheetcheck => "sheetcheck",
            TicketType::Staff => "staff",
            TicketType::Lore => "lore",
            TicketType::Homebrew => "homebrew",
            TicketType::HbItem => "item",
            TicketType::HbFeat => "feat",
            TicketType::HbSpell => "spell",
            TicketType::HbOther => "other",
            TicketType::HbSubclass => "subclass",
        }
        .to_string()
    }
    pub fn get_modal_id(&self) -> &str {
        match self {
            TicketType::Character | TicketType::Respec => unimplemented!(),
            TicketType::Dm => "dm_ticket_modal",
            TicketType::Sheetcheck => "sheetcheck_ticket_modal",
            TicketType::Staff => "staff_ticket_modal",
            TicketType::Lore => "lore_ticket_modal",
            TicketType::Homebrew => "homebrew_ticket_modal",
            TicketType::HbItem => "hb_item_modal",
            TicketType::HbFeat => "hb_feat_modal",
            TicketType::HbSpell => "hb_spell_modal",
            TicketType::HbOther => "hb_other_modal",
            TicketType::HbSubclass => "hb_subclass_modal",
        }
    }
    pub fn get_title(&self) -> String {
        match &self {
            TicketType::Character => todo!(),
            TicketType::Respec => todo!(),
            TicketType::Dm => "DM Application",
            TicketType::Sheetcheck => "Sheet Checker Application",
            TicketType::Staff => "Staff Application",
            TicketType::Lore => "Lore Team Application",
            TicketType::Homebrew => "Homebrew Team Application",
            TicketType::HbItem => "Homebrew Item Application",
            TicketType::HbFeat => "Homebrew Feat Application",
            TicketType::HbSpell => "Homebrew Spell Application",
            TicketType::HbOther => "Homebrew Other Application",
            TicketType::HbSubclass => "Homebrew Subclass Application",
        }
        .to_string()
    }
    pub fn get_embed(self) -> CreateEmbed {
        match self {
            TicketType::Character => character::embed(),
            TicketType::Respec => respec::embed(),
            TicketType::Dm => dm::embed(),
            TicketType::Sheetcheck => sheetcheck::embed(),
            TicketType::Staff => staff::embed(),
            TicketType::Lore => lore::embed(),
            TicketType::Homebrew => homebrew::embed(),
            TicketType::HbItem => hb_item::embed(),
            TicketType::HbFeat => hb_feat::embed(),
            TicketType::HbSpell => hb_spell::embed(),
            TicketType::HbSubclass => hb_subclass::embed(),
            TicketType::HbOther => hb_other::embed()
        }
    }
    pub fn get_category(self) -> SecretType {
        match self {
            TicketType::HbSubclass
            | TicketType::HbItem
            | TicketType::HbSpell
            | TicketType::HbOther
            | TicketType::HbFeat => SecretType::HomebrewCategoryId,
            _ => SecretType::CategoryId
        }
    }
}

pub async fn open_modal(
    ctx: &Context,
    command: &ComponentInteraction,
    ticket_type: TicketType,
) -> Result<String, String> {
    send_modal(
        ticket_type.get_modal_id().to_string(),
        &ctx,
        &command,
        &match ticket_type {
            TicketType::Dm => dm::get_questions(),
            TicketType::Sheetcheck => sheetcheck::get_questions(),
            TicketType::Lore => lore::get_questions(),
            TicketType::Staff => staff::get_questions(),
            TicketType::Homebrew => homebrew::get_questions(),
            TicketType::HbItem => hb_item::get_questions(),
            TicketType::HbFeat => hb_feat::get_questions(),
            TicketType::HbSpell => hb_spell::get_questions(),
            TicketType::HbOther => hb_other::get_questions(),
            TicketType::HbSubclass => hb_subclass::get_questions(),
            _ => unimplemented!(),
        },
        ticket_type.get_title(),
    )
    .await;

    Ok("Modal Opened".to_string())
}

pub async fn create_ticket(
    ctx: &Context,
    ticket_type: TicketType,
    information: TicketInformation,
) -> Result<String, String> {
    let bot_role = config::get_config_val(config::SecretType::BotRole)
        .parse::<u64>()
        .map_err(|_| "Unable to parse the retrieved Staff role ID into u64".to_string())?;

    let staff_role = config::get_config_val(config::SecretType::Staff)
        .parse::<u64>()
        .map_err(|_| "Unable to parse the retrieved Staff role ID into u64".to_string())?;

    let ticket_role = config::get_config_val(SecretType::from(ticket_type))
        .parse::<u64>()
        .map_err(|_| "Unable to retrieve the retrieved role ID into u64")?;

    let category_id = config::get_config_val(ticket_type.get_category())
        .parse::<u64>()
        .map_err(|_| "Unable to parse the retrieved category ID".to_string())?;

    let mut perms = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(RoleId::new(bot_role)),
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(RoleId::new(information.guild_id)),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(RoleId::new(staff_role)),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(RoleId::new(ticket_role)),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(information.author),
        },
    ];
    if vec![TicketType::HbFeat, TicketType::HbItem, TicketType::HbSpell, TicketType::HbOther, TicketType::HbSubclass]
        .contains(&TicketType::from(ticket_type))
    {
        perms.get_mut(4).unwrap().allow = Permissions::empty();
        perms.get_mut(4).unwrap().deny = Permissions::VIEW_CHANNEL;
    }

    if information
        .guild
        .channels(&ctx.http)
        .await
        .map_err(|_| "Unable to fetch channel names from the server".to_string())?
        .iter()
        .filter(|(_, val)| {
            val.name
                == format!(
                    "{}-{}",
                    ticket_type.to_string(),
                    information.author_name.to_lowercase()
                )
        })
        .count()
        > 0
    {
        return Err("You already have a ticket of this type open. Please have it closed before you open another.".to_string());
    }

    let channel = information
        .guild
        .create_channel(
            &ctx.http,
            CreateChannel::new(format!(
                "{}-{}",
                &ticket_type.to_string(),
                information.author_name
            ))
            .category(ChannelId::new(category_id))
            .permissions(perms),
        )
        .await
        .map_err(|_| "Unable to create the channel for the ticket".to_string())?;


    let mut embeds = vec![ticket_type.get_embed()];
    if information.questions.len() > 0 {
        embeds.push(CreateEmbed::new().title("Submitted Answers").color(Color::RED)
            .fields(
                information.questions.iter().clone().map( |val|
                    (&val.title, &val.answer, false)
                )
            ));
    }

    channel
        .send_message(&ctx.http, CreateMessage::new()
            .content(format!("<@{}> <@&{}>", information.author_id, ticket_role))
            .embeds(embeds))
        .await
        .map_err(|_| format!("Unable to make the message post in {}", channel.id.get()))?;
    Ok(channel.id.get().to_string())
}

pub async fn create_ticket_from_modal(
    ctx: &Context,
    submission: &ModalInteraction,
    ticket_type: TicketType,
) -> Result<String, String> {
    let author = submission.user.name.to_owned();

    let author_id = submission.user.id.get();

    let guild_id = submission.guild_id.unwrap().get();

    let guild_id_interactive = submission
        .guild_id
        .ok_or("Unable to retrieve the server from the bot".to_string())?;

    let mut questions: Vec<Question> = Vec::new();

    submission.data.components.iter().for_each(|f| {
        f.components.iter().for_each(|g| match g {
            ActionRowComponent::InputText(z) => questions.push(Question {
                title: get_question_from_id(z.custom_id.as_str()),
                answer: z.clone().value.unwrap().to_string(),
            }),
            ActionRowComponent::Button(_) => {}
            ActionRowComponent::SelectMenu(_) => {}
            _ => {}
        })
    });

    if vec![TicketType::HbSpell, TicketType::HbOther, TicketType::HbItem, TicketType::HbFeat, TicketType::HbSubclass]
        .contains(&ticket_type) &&
        questions.clone().into_iter().filter(|x| Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}(\.[a-z]{2,4})?\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)").unwrap().is_match(&x.answer)).count() == 0
    {
        Err("No valid URL found")?
    }

    let data: TicketInformation = TicketInformation {
        ticket_type,
        author_name: author,
        author: submission.user.id,
        author_id,
        guild_id,
        guild: guild_id_interactive,
        questions: questions.clone(),
    };

    create_ticket(&ctx, ticket_type, data).await
}

pub async fn run(
    ctx: &Context,
    command: &ComponentInteraction,
    ticket_type: TicketType,
) -> Result<String, String> {
    let author = command.user.name.to_owned();

    let author_id = command.user.id.get();

    let guild_id = command.guild_id.unwrap().get();

    let guild_id_interactive = command
        .guild_id
        .ok_or("Unable to retrieve the server from the bot".to_string())?;

    let data: TicketInformation = TicketInformation {
        ticket_type,
        author_name: author,
        author: command.user.id,
        author_id,
        guild_id,
        guild: guild_id_interactive,
        questions: Vec::new(),
    };

    create_ticket(&ctx, ticket_type, data).await
}
