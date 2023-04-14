mod embeds;
use std::str::FromStr;

use serenity::{
    builder::CreateInteractionResponse,
    model::{
        prelude::{
            interaction::{
                message_component::MessageComponentInteraction, InteractionResponseType,
            },
            ChannelId, PermissionOverwrite, PermissionOverwriteType, RoleId,
        },
        Permissions,
    },
    prelude::Context,
};

use crate::config::{self, SecretType};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TicketType {
    Character,
    Respec,
    Dm,
    Sheetcheck,
    Shopkeep,
    Staff,
}

impl FromStr for TicketType {
    type Err = ();
    fn from_str(input: &str) -> Result<TicketType, ()> {
        match input {
            "create_character_ticket" => Ok(TicketType::Character),
            "create_respec_ticket" => Ok(TicketType::Respec),
            "create_dm_ticket" => Ok(TicketType::Dm),
            "create_sheetcheck_ticket" => Ok(TicketType::Sheetcheck),
            "create_shopkeep_ticket" => Ok(TicketType::Shopkeep),
            "create_staff_ticket" => Ok(TicketType::Staff),
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
            TicketType::Shopkeep => "shopkeep",
            TicketType::Staff => "staff",
        }
        .to_string()
    }
}

pub async fn open_modal(
    ctx: &Context,
    command: &MessageComponentInteraction,
    ticket_type: TicketType,
) {
    match ticket_type {
        TicketType::Dm => self::embeds::dm::get_modal(&command),
        TicketType::Sheetcheck => self::embeds::sheetcheck::get_modal(&command),
        TicketType::Shopkeep => self::embeds::shopkeep::get_modal(&command),
        TicketType::Staff => self::embeds::staff::get_modal(&command),
        _ => unreachable!(),
    }
}

pub async fn run(
    ctx: &Context,
    command: &MessageComponentInteraction,
    ticket_type: TicketType,
) -> Result<String, String> {
    let author = command.user.name.to_owned();

    let author_id = command.user.id.0;

    let staff_role = config::get_config_val(config::SecretType::Staff)
        .parse::<u64>()
        .map_err(|_| "Unable to parse the retrieved Staff role ID into u64".to_string())?;

    let ticket_role = config::get_config_val(SecretType::from(ticket_type))
        .parse::<u64>()
        .map_err(|_| "Unable to retrieve the retrieved role ID into u64")?;

    let category_id = config::get_config_val(SecretType::CategoryId)
        .parse()
        .map_err(|_| "Unable to parse the retrieved category ID".to_string())?;

    let perms = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(RoleId(staff_role)),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(RoleId(ticket_role)),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(command.user.id),
        },
    ];

    let channel = command
        .guild_id
        .ok_or("Unable to get the retrieve the server from the bot".to_string())?
        .create_channel(&ctx.http, |channel| {
            channel
                .category(ChannelId(category_id))
                .name(format!("{}-{}", &ticket_type.to_string(), author))
                .permissions(perms)
        })
        .await
        .map_err(|_| "Unable to create the channel for the ticket".to_string())?;

    channel
        .send_message(&ctx.http, |message| {
            message
                .content(format!("<@{}> <@&{}>", author_id, ticket_role))
                .embed(|e| match ticket_type {
                    TicketType::Character => embeds::character::embed(e),
                    TicketType::Respec => embeds::respec::embed(e),
                    TicketType::Dm => embeds::dm::embed(e),
                    TicketType::Sheetcheck => embeds::sheetcheck::embed(e),
                    TicketType::Shopkeep => embeds::shopkeep::embed(e),
                    TicketType::Staff => embeds::staff::embed(e),
                })
        })
        .await
        .map_err(|_| format!("Unable to make the message post in {}", channel.id.0))?;

    Ok(channel.id.0.to_string())
}
