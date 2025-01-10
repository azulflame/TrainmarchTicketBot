use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use crate::config;
use chrono::{DateTime, NaiveDateTime, Utc};
use serenity::all::{
    ChannelId, CommandInteraction, CommandOptionType, CreateAttachment, CreateMessage, Embed,
    GetMessages, GuildChannel, GuildId, Message, Permissions, Role, RoleId, User,
};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::Colour;
use serenity::prelude::*;

pub async fn run(ctx: &Context, command: &CommandInteraction) -> Result<String, String> {
    let author_id = &command.user.id.get();
    // Verify the reason is a valid string
    let log_channel_id = config::get_config_val(config::SecretType::LogChannel)
        .parse::<u64>()
        .map_err(|_| "Unable to parse the provided log channel ID".to_string())?;

    let log_channel = &command
        .guild_id
        .ok_or("Unable to get the Guild from the command".to_string())?
        .channels(&ctx.http)
        .await
        .map_err(|_| "Unable to get the channels for the guild".to_string())?
        .get(ChannelId::new(log_channel_id).as_ref())
        .ok_or("Unable to find the log channel".to_string())?
        .clone();

    let _reason = &command
        .data
        .options
        .get(0)
        .ok_or("Expected a closure message, but one was not found".to_string())?
        .value
        .as_str()
        .ok_or("Expected a closure message, but one was not found".to_string())?;

    let channel = command
        .channel_id
        .to_channel(&ctx.http)
        .await
        .map_err(|_| "Unable to retrieve the channel the message was sent in.".to_string())?
        .guild()
        .ok_or("Unable to retrieve the Guild for the channel.".to_string())?;

    // Verify the category matches the provided category ID
    let category_id = &channel
        .parent_id
        .ok_or("Unable to get the Category for the provided Channel.".to_string())?
        .get();

    let expected_category_id = config::get_config_val(config::SecretType::CategoryId)
        .parse::<u64>()
        .map_err(|_| "Unable to parse the category ID into an u64".to_string())?;

    if *category_id != expected_category_id {
        return Err("This is not a ticket channel.".to_string());
    }

    // Now that we are in the right place and have a reason, we can pull the logs and close the ticket

    // pull logs

    let messages = command
        .channel_id
        .messages(&ctx.http, GetMessages::new().limit(100))
        .await
        .map_err(|_| {
            format!(
                "Unable to get the logs for the channel <#{}>",
                channel.name()
            )
        })?;

    // Delete the channel

    let deleted_channel = &command
        .channel_id
        .delete(&ctx.http)
        .await
        .map_err(|_| "The channel could not be deleted.".to_string())?
        .guild()
        .unwrap()
        .name;

    // Post the logs

    // Create the log first
    let filename = format!("{}-{}.html", &deleted_channel, &channel.id.get()).to_string();
    let mut f =
        File::create(&filename).map_err(|_| "Error writing log file before upload".to_string())?;
    f.write_all(parse_logs(messages, &ctx).await.as_bytes())
        .map_err(|_| "Error writing log file before upload".to_string())?;
    f.flush()
        .map_err(|_| "Error writing log file before upload".to_string())?;
    log_channel
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .content(format!(
                    "{} closed by <@{}> for reason \"{}\".",
                    deleted_channel, author_id, _reason
                ))
                .add_file(CreateAttachment::path(filename.as_str()).await.unwrap()),
        )
        .await
        .map_err(|_| "Error posting log message to the log channel".to_string())?;

    fs::remove_file(filename).map_err(|_| "Unable to delete the log file.")?;

    Ok(format!("Ticket {} closed.", &channel.name()))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("close")
        .description("close a ticket!")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "reason", "The reason for closing the ticket. Defaults to \"The ticket was closed and no reason was given.\"")
                .required(true)
        )
        .default_member_permissions(Permissions::MANAGE_ROLES)
}

async fn parse_logs(inp: Vec<Message>, ctx: &Context) -> String {
    let guild = GuildId::new(
        config::get_config_val(config::SecretType::GuildId)
            .parse()
            .unwrap(),
    );
    let channels = guild
        .channels(&ctx.http)
        .await
        .expect("Unable to fetch channels");
    let role_list = guild
        .roles(&ctx.http)
        .await
        .expect("Unable to retrieve roles");
    format!(
        "<!DOCTYPE html><html><head><script src=\"https://unpkg.com/wc-discord-message@^2.0.0/dist/wc-discord-message/wc-discord-message.js\"></script>
        </head><body><discord-messages>{}</discord-messages></body></html>",
        inp.iter()
            .map(|f|
               format!(
                        "<discord-message author=\"{}\" avatar=\"{}\" {} {} timestamp=\"{}\">{}{}</discord-message>",
                        f.author.name,
                        f.author.avatar_url().unwrap_or_else(||"https://cdn.discordapp.com/attachments/654503812593090602/665721745466195978/blue.png".to_string()),
                        match f.author.bot{true => "bot", false => ""},
                        match f.edited_timestamp.is_some() {true => "edited", false => ""},
                        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(match f.edited_timestamp {Some(x) => x,
                            None => f.timestamp}.unix_timestamp(), 0).expect("Invalid Timestamp"), Utc).format("%m/%d/%Y"),
                        process_to_wc_discord_format(f.content.replace("\n", "<br>"), &role_list, &channels, &f.mentions),
                        format_embed(&f.embeds, &role_list, &channels, &f.mentions),
            ))
            .fold("".to_string(), |acc, val| format!("{}{}", val, acc))
    )
}

fn format_embed(
    vec: &Vec<Embed>,
    roles: &HashMap<RoleId, Role>,
    channels: &HashMap<ChannelId, GuildChannel>,
    users: &Vec<User>,
) -> String {
    vec.iter()
        .map(|e| {
            format!(
                "<discord-embed slot=\"embeds\" color=\"{}\" embedTitle=\"{}\">{}{}</discord-embed>",
                e.colour.unwrap_or_else(|| Colour::new(0)).hex(),
                process_to_wc_discord_format(e.title.as_ref().map_or("".to_string(), |f| f.to_string()), &roles, &channels, &users),
                process_to_wc_discord_format(e.description.clone().unwrap_or_else(|| "".to_string()).replace("\\n", "<br>"), &roles, &channels, &users),
                match e
                    .fields
                    .iter()
                    .map(|f| format!(
                        "<discord-embed-field field-title=\"{}\">{}</discord-embed-field>",
                        f.name, process_to_wc_discord_format(f.value.replace("\\n", "<br>"), &roles, &channels, &users)
                    ))
                    .fold("".to_string(), |a, b| format!("{}{}", a, b))
                    .as_str()
                {
                    "" => "".to_string(),
                    x => format!("<discord-embed-fields>{}</discord-embed-fields>", x),
                }
            )
        })
        .fold("".to_string(), |a, b| format!("{}{}", a, b))
}

fn process_to_wc_discord_format(
    str: String,
    roles: &HashMap<RoleId, Role>,
    channels: &HashMap<ChannelId, GuildChannel>,
    users: &Vec<User>,
) -> String {
    let mut output = str.clone();
    for (role_id, role) in roles {
        output = output.replace(
            format!("<@&{}>", role_id).as_str(),
            format!(
                "<discord-mention type=\"role\" color=\"{}\">{}</discord-mention>",
                role.colour.hex(),
                role.name
            )
            .as_str(),
        );
    }
    for (channel_id, channel) in channels {
        output = output.replace(
            format!("<#{}>", channel_id).as_str(),
            format!(
                "<discord-mention type=\"channel\">{}</discord-mention>",
                channel.name
            )
            .as_str(),
        );
    }
    for user in users {
        output = output.replace(
            format!("<@{}>", user.id).as_str(),
            format!(
                "<discord-mention type=\"user\">{}</discord-mention>",
                user.name
            )
            .as_str(),
        );
    }

    output
}
