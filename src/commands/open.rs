mod embeds;
use std::str::FromStr;

use serenity::{
    model::{
        prelude::{
            component::ActionRowComponent,
            interaction::{
                message_component::MessageComponentInteraction, modal::ModalSubmitInteraction,
            },
            ChannelId, PermissionOverwrite, PermissionOverwriteType, RoleId,
        },
        Permissions,
    },
    prelude::Context,
    utils::Color,
};

use crate::config::{self, SecretType};

use self::embeds::{dm, get_question_from_id, lore, send_modal, sheetcheck, shopkeep, staff};

#[derive(Debug, Clone)]
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
    Shopkeep,
    Staff,
    Lore,
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
            "create_lore_ticket" => Ok(TicketType::Lore),
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
            TicketType::Lore => "lore",
        }
        .to_string()
    }
    pub fn get_modal_id(&self) -> &str {
        match self {
            TicketType::Character | TicketType::Respec => unimplemented!(),
            TicketType::Dm => "dm_ticket_modal",
            TicketType::Sheetcheck => "sheetcheck_ticket_modal",
            TicketType::Shopkeep => "shopkeep_ticket_modal",
            TicketType::Staff => "staff_ticket_modal",
            TicketType::Lore => "lore_ticket_modal",
        }
    }
    pub fn get_title(&self) -> String {
        match &self {
            TicketType::Character => todo!(),
            TicketType::Respec => todo!(),
            TicketType::Dm => "DM Application",
            TicketType::Sheetcheck => "Sheetchecker Application",
            TicketType::Shopkeep => "Shopkeep Application",
            TicketType::Staff => "Staff Application",
            TicketType::Lore => "Lore Team Application",
        }
        .to_string()
    }
}

pub async fn open_modal(
    ctx: &Context,
    command: &MessageComponentInteraction,
    ticket_type: TicketType,
) -> Result<String, String> {
    match ticket_type {
        TicketType::Lore => {
            send_modal(
                ticket_type.get_modal_id().to_string(),
                &ctx,
                &command,
                lore::LORE_QUESTIONS.as_ref(),
                ticket_type.get_title(),
            )
            .await
        }
        TicketType::Dm => {
            send_modal(
                ticket_type.get_modal_id().to_string(),
                &ctx,
                &command,
                dm::DM_QUESTIONS.as_ref(),
                ticket_type.get_title(),
            )
            .await
        }
        TicketType::Sheetcheck => {
            send_modal(
                ticket_type.get_modal_id().to_string(),
                &ctx,
                &command,
                sheetcheck::SHEETCHECK_QUESTIONS.as_ref(),
                ticket_type.get_title(),
            )
            .await
        }
        TicketType::Staff => {
            send_modal(
                ticket_type.get_modal_id().to_string(),
                &ctx,
                &command,
                staff::STAFF_QUESTIONS.as_ref(),
                ticket_type.get_title(),
            )
            .await
        }
        TicketType::Shopkeep => {
            send_modal(
                ticket_type.get_modal_id().to_string(),
                &ctx,
                &command,
                shopkeep::SHOPKEEP_QUESTIONS.as_ref(),
                ticket_type.get_title(),
            )
            .await
        }
        TicketType::Character => unimplemented!(),
        TicketType::Respec => unimplemented!(),
    };
    Ok("Modal Opened".to_string())
}

pub async fn create_ticket_from_modal(
    ctx: &Context,
    submission: &ModalSubmitInteraction,
    ticket_type: TicketType,
) -> Result<String, String> {
    let author = submission.user.name.to_owned();

    let author_id = submission.user.id.0;

    let guild_id = submission.guild_id.unwrap().0;

    let staff_role = config::get_config_val(config::SecretType::Staff)
        .parse::<u64>()
        .map_err(|_| "Unable to parse the retrieved Staff role ID into u64".to_string())?;

    let ticket_role = config::get_config_val(SecretType::from(ticket_type))
        .parse::<u64>()
        .map_err(|_| "Unable to retrieve the retrieved role ID into u64")?;

    let category_id = config::get_config_val(SecretType::CategoryId)
        .parse::<u64>()
        .map_err(|_| "Unable to parse the retrieved category ID".to_string())?;

    let perms = vec![
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(RoleId(guild_id)),
        },
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
            kind: PermissionOverwriteType::Member(submission.user.id),
        },
    ];

    let channel = submission
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

    let mut questions: Vec<Question> = Vec::new();

    submission.data.components.iter().for_each(|f| {
        f.components.iter().for_each(|g| match g {
            ActionRowComponent::InputText(z) => questions.push(Question {
                title: get_question_from_id(z.custom_id.as_str()),
                answer: z.value.to_string(),
            }),
            ActionRowComponent::Button(_) => {}
            ActionRowComponent::SelectMenu(_) => {}
            _ => {}
        })
    });

    channel
        .send_message(&ctx.http, |message| {
            message
                .content(format!("<@{}> <@&{}>", author_id, ticket_role))
                .add_embed(|e| match ticket_type {
                    TicketType::Character => embeds::character::embed(e).color(Color::BLUE),
                    TicketType::Respec => embeds::respec::embed(e).color(Color::BLUE),
                    TicketType::Dm => embeds::dm::embed(e).color(Color::BLUE),
                    TicketType::Sheetcheck => embeds::sheetcheck::embed(e).color(Color::BLUE),
                    TicketType::Shopkeep => embeds::shopkeep::embed(e).color(Color::BLUE),
                    TicketType::Staff => embeds::staff::embed(e).color(Color::BLUE),
                    TicketType::Lore => embeds::lore::embed(e).color(Color::BLUE),
                })
                .add_embed(|e| {
                    questions.iter().clone().for_each(|val| {
                        e.field(&val.title, &val.answer, false);
                    });
                    e.title("Submitted Answers").color(Color::RED)
                })
        })
        .await
        .map_err(|_| format!("Unable to make the message post in {}", channel.id.0))?;

    Ok("".to_string())
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
                    TicketType::Lore => embeds::lore::embed(e).color(Color::ORANGE),
                    TicketType::Character => embeds::character::embed(e).color(Color::ORANGE),
                    TicketType::Respec => embeds::respec::embed(e).color(Color::ORANGE),
                    TicketType::Dm => embeds::dm::embed(e).color(Color::ORANGE),
                    TicketType::Sheetcheck => embeds::sheetcheck::embed(e).color(Color::ORANGE),
                    TicketType::Shopkeep => embeds::shopkeep::embed(e).color(Color::ORANGE),
                    TicketType::Staff => embeds::staff::embed(e).color(Color::ORANGE),
                })
        })
        .await
        .map_err(|_| format!("Unable to make the message post in {}", channel.id.0))?;

    Ok(channel.id.0.to_string())
}
