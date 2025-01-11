use serenity::all::{ChannelId, Colour, CommandInteraction, CommandOptionType, Context, CreateAttachment, CreateCommand, CreateCommandOption, CreateEmbed, CreateMessage, GetMessages, Permissions};
use crate::config;

pub fn register() -> CreateCommand {
    CreateCommand::new("homebrew-close")
        .description("close a homebrew ticket")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Boolean, "approved", "If the ticket is approved")
                .required(true)
        )
        .add_option(CreateCommandOption::new(CommandOptionType::String, "reason", "Why the approval state is what it is (If denied, this is why)")
            .required(true))
        .add_option(CreateCommandOption::new(CommandOptionType::User, "submitter", "The original submitter")
            .required(true))
        .add_option(CreateCommandOption::new(CommandOptionType::String, "link", "The link to the homebrew")
            .required(true))
        .default_member_permissions(Permissions::MANAGE_ROLES)
}

pub async fn run(ctx: &Context, command: &CommandInteraction) -> Result<String, String> {
    let author_id = &command.user.id.get();
    // Verify the reason is a valid string
    let log_channel_id = config::get_config_val(config::SecretType::HomebrewLogChannel)
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

    let link = &command
        .data
        .options
        .iter()
        .filter(|x| x.name == "link")
        .collect::<Vec<_>>()
        .get(0)
        .ok_or("Unable to get the link".to_string())?
        .value
        .as_str()
        .ok_or("Unable to parse the link".to_string())?;

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

    let embed_color = match *approval {
        true => Colour::DARK_GREEN,
        false => Colour::RED,
    };


    log_channel
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .content(format!(
                    "<@{}>",
                    who_to_ping
                ))
                .embed(
                    CreateEmbed::new()
                        .title("Homebrew Submission")
                        .field("Approved:", match approval {true => "Yes", false => "No"}, false)
                        .field("Link:", format!("<{}>", link), false)
                        .field("Reason:", _reason, false)
                        .color(embed_color)
                )
        )
        .await
        .map_err(|_| "Error posting log message to the log channel".to_string())?;

    Ok(format!("Ticket {} closed.", &channel.name()))
}