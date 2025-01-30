use std::fs;
use std::fs::File;
use std::io::Write;
use serenity::all::{ChannelId, Colour, CommandInteraction, CommandOptionType, Context, CreateAttachment, CreateCommand, CreateCommandOption, CreateEmbed, CreateMessage, GetMessages, Permissions};
use crate::commands::close::{get_messages, parse_logs};
use crate::config;
use crate::config::{get_config_val, SecretType};

pub fn register() -> CreateCommand {
    CreateCommand::new("homebrew-close")
        .description("close a homebrew ticket")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Boolean, "approved", "If the ticket is approved")
                .required(true)
        )
        .add_option(CreateCommandOption::new(CommandOptionType::String, "reason", "Why the approval state is what it is (If denied, this is why). printed into the transcript channel")
            .required(true).max_length(1000))
        .add_option(CreateCommandOption::new(CommandOptionType::User, "submitter", "The original submitter")
            .required(true))
        .default_member_permissions(Permissions::MANAGE_ROLES)
}

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

    let hb_result_id = get_config_val(SecretType::HomebrewLogChannel).parse::<u64>().map_err(|_| "Unable to parse the provided log channel ID".to_string())?;

    let hb_result_channel = &command
        .guild_id
        .ok_or("Unable to get the Guild from the command".to_string())?
        .channels(&ctx.http)
        .await
        .map_err(|_| "Unable to get the channels for the guild".to_string())?
        .get(ChannelId::new(hb_result_id).as_ref())
        .ok_or("Unable to find the log channel".to_string())?
        .clone();

    let _reason = command
        .data
        .options.iter().filter(|x| x.name == "reason").collect::<Vec<_>>()
        .get(0)
        .ok_or("Unable to get the reason".to_string())?
        .value
        .as_str()
        .ok_or("Expected a closure message, but one was not found".to_string())?;

    let approval = &command
        .data
        .options.iter().filter(|x| x.name == "approved").collect::<Vec<_>>()
        .get(0)
        .ok_or("Unable to get the approval state".to_string())?
        .value
        .as_bool()
        .ok_or("Unable to parse the approval state".to_string())?;

    let who_to_ping = &command
        .data
        .options
        .iter()
        .filter(|x| x.name == "submitter")
        .collect::<Vec<_>>()
        .get(0)
        .ok_or("Unable to identify who to ping".to_string())?
        .value.as_user_id()
        .ok_or("Unable to parse the User ID")?;

    let channel = &command
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

    let homebrew_category_id = config::get_config_val(config::SecretType::HomebrewCategoryId)
        .parse::<u64>()
        .map_err(|_| "Unable to parse the category ID into an u64".to_string())?;

    if *category_id != homebrew_category_id{
        return Err("This is not a ticket channel.".to_string());
    }

    // Now that we are in the right place and have a reason, we can pull the logs and close the ticket
    let messages = get_messages(&channel, &ctx).await?;

    let first_message = messages.get(0);
    if first_message.is_none() {
        return Err("Unable to retrieve the first message".to_string());
    }

    println!("{:?}", first_message);

    let fields = first_message.unwrap()
        .embeds
        .get(1)
        .ok_or_else(||
            "Unable to get first embeds"
                .to_string()
        )?
        .fields
        .iter()
        .map(|f|
            (&f.name, &f.value, f.inline)
        )
        .collect::<Vec<_>>();

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
    f.write_all(parse_logs(messages.clone(), &ctx).await.as_bytes())
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

    // Post the voting result

    if *approval {
        let embed_color = match *approval {
            true => Colour::DARK_GREEN,
            false => Colour::RED,
        };

        hb_result_channel
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .content(format!(
                        "<@{}>",
                        who_to_ping
                    ))
                    .embed(
                        CreateEmbed::new()
                            .title("Homebrew Submission Approved")
                            .fields(fields)
                            .color(embed_color)
                    )
            )
            .await
            .map_err(|_| "Error posting log message to the log channel".to_string())?;
    }
    Ok(format!("Ticket {} closed.", &channel.name()))
}