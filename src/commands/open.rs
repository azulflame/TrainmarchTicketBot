use std::str::FromStr;

use serenity::{
    builder::CreateEmbed,
    model::{
        prelude::{
            component::ActionRowComponent,
            interaction::{
                message_component::MessageComponentInteraction, modal::ModalSubmitInteraction,
            },
            ChannelId, GuildId, PermissionOverwrite, PermissionOverwriteType, RoleId, UserId,
        },
        Permissions,
    },
    prelude::Context,
    utils::Color,
};

use crate::config::{self, SecretType};

use super::tickets::{dm, get_question_from_id, lore, send_modal, sheetcheck, staff, homebrew, character, respec};

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
            TicketType::Homebrew => "homebrew"
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
        }
    }
    pub fn get_title(&self) -> String {
        match &self {
            TicketType::Character => todo!(),
            TicketType::Respec => todo!(),
            TicketType::Dm => "DM Application",
            TicketType::Sheetcheck => "Player Affairs Team Application",
            TicketType::Staff => "Staff Application",
            TicketType::Lore => "Lore Team Application",
            TicketType::Homebrew => "Homebrew Team Application",
        }
        .to_string()
    }
    pub fn get_embed(self, e: &mut CreateEmbed) -> &mut CreateEmbed {
        match self {
            TicketType::Character => character::embed(e),
            TicketType::Respec => respec::embed(e),
            TicketType::Dm => dm::embed(e),
            TicketType::Sheetcheck => sheetcheck::embed(e),
            TicketType::Staff => staff::embed(e),
            TicketType::Lore => lore::embed(e),
            TicketType::Homebrew => homebrew::embed(e)
        }
    }
}

pub async fn open_modal(
    ctx: &Context,
    command: &MessageComponentInteraction,
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
            _ => unimplemented!(),

        },
        ticket_type.get_title()
        ).await;

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

    let category_id = config::get_config_val(SecretType::CategoryId)
        .parse::<u64>()
        .map_err(|_| "Unable to parse the retrieved category ID".to_string())?;

    let perms = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(RoleId(bot_role)),
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(RoleId(information.guild_id)),
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
            kind: PermissionOverwriteType::Member(information.author),
        },
    ];

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
        .create_channel(&ctx.http, |channel| {
            channel
                .category(ChannelId(category_id))
                .name(format!(
                    "{}-{}",
                    &ticket_type.to_string(),
                    information.author_name
                ))
                .permissions(perms)
        })
        .await
        .map_err(|_| "Unable to create the channel for the ticket".to_string())?;

    channel
        .send_message(&ctx.http, |message| {
            let mut m = message
                .content(format!("<@{}> <@&{}>", information.author_id, ticket_role))
                .add_embed(|e| ticket_type.get_embed(e));
            if information.questions.len() > 0 {
                m = m.add_embed(|e| {
                    information.questions.iter().clone().for_each(|val| {
                        e.field(&val.title, &val.answer, false);
                    });
                    e.title("Submitted Answers").color(Color::RED)
                });
            }
            m
        })
        .await
        .map_err(|_| format!("Unable to make the message post in {}", channel.id.0))?;
    Ok(channel.id.0.to_string())
}

pub async fn create_ticket_from_modal(
    ctx: &Context,
    submission: &ModalSubmitInteraction,
    ticket_type: TicketType,
) -> Result<String, String> {
    let author = submission.user.name.to_owned();

    let author_id = submission.user.id.0;

    let guild_id = submission.guild_id.unwrap().0;

    let guild_id_interactive = submission
        .guild_id
        .ok_or("Unable to retrieve the server from the bot".to_string())?;

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

    let data: TicketInformation = TicketInformation {
        ticket_type: ticket_type,
        author_name: author,
        author: submission.user.id,
        author_id: author_id,
        guild_id: guild_id,
        guild: guild_id_interactive,
        questions: questions.clone(),
    };

    create_ticket(&ctx, ticket_type, data).await
}

pub async fn run(
    ctx: &Context,
    command: &MessageComponentInteraction,
    ticket_type: TicketType,
) -> Result<String, String> {
    let author = command.user.name.to_owned();

    let author_id = command.user.id.0;

    let guild_id = command.guild_id.unwrap().0;

    let guild_id_interactive = command
        .guild_id
        .ok_or("Unable to retrieve the server from the bot".to_string())?;

    let data: TicketInformation = TicketInformation {
        ticket_type: ticket_type,
        author_name: author,
        author: command.user.id,
        author_id: author_id,
        guild_id: guild_id,
        guild: guild_id_interactive,
        questions: Vec::new(),
    };

    create_ticket(&ctx, ticket_type, data).await
}
