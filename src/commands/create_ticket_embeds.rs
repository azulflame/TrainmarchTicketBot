use serenity::builder::{CreateApplicationCommand, CreateButton};
use serenity::client::Context;
use serenity::model::application::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::*;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<String, String> {
    command
        .channel_id
        .send_message(&ctx.http, |message_builder| {
            message_builder
            .embed(|embed_builder| {
                embed_builder
                .title("Character Applications")
                .field("Open an Application", "Click the button below to open a character or respec ticket. Instructions can be found in <#821929650753634314>, or in the messages in the ticket.", false)
            })
            .components(|c| {
                c.create_action_row(|row| {
                    row
                    .add_button(CreateButton::default().custom_id("create_character_ticket").label("Create Character").style(ButtonStyle::Primary).to_owned())
                    .add_button(CreateButton::default().custom_id("create_respec_ticket").label("Respec Character").style(ButtonStyle::Primary).to_owned())
                })
            })
        })
        .await.map_err(|_| "And error was encountered creating the Character embed".to_string())?;

        command
            .channel_id
            .send_message(&ctx.http, |message_builder| {
                message_builder
                    .embed(|embed_builder| {
                        embed_builder
                            .title("Server Applications")
                            .field("Open a Server Application", "These applications are for the roles that aid the server to run. Each ticket will have ", false)
                    })
                .components(|c| {
                    c.create_action_row(|r|
                                        {
                                            r.add_button(CreateButton::default().custom_id("create_dm_ticket").label("Dungeon Master").style(ButtonStyle::Primary).to_owned())
                                                .add_button(CreateButton::default().custom_id("create_homebrew_ticket").label("Homebrew Team").style(ButtonStyle::Primary).to_owned())
                                                .add_button(CreateButton::default().custom_id("create_sheetcheck_ticket").label("Sheet Checker").style(ButtonStyle::Primary).to_owned())
                                        }
                                       )
                })
            }).await.map_err(|_|"An error was encountered creating the Server Applications embed".to_string())?;

        command.channel_id.send_message(&ctx.http, |message_builder| {
            message_builder.embed(|embed_builder| {
                embed_builder.title("Server Applications")
                    .title("Server Management Applications")
                    .field("Open an Application", "These tickets are for the roles that manage the server.", false)
                    .field("Instructions", "Each ticket will have instructions and questions inside of the ticket after you open it.", false)
            })
            .components(|c| {
                c.create_action_row(|r|
                                    {
                                        r.add_button(CreateButton::default().custom_id("create_staff_ticket").label("Staff").style(ButtonStyle::Primary).to_owned())
                                            .add_button(CreateButton::default().custom_id("create_lore_ticket").label("Lore Team").style(ButtonStyle::Primary).to_owned())
                                    })
            })
        }).await.map_err(|_| "And error was encountered creating the Server Management embed".to_string())?;

        Ok("Embeds Created!".to_string())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("create-ticket-posts")
        .description("Create the embeds and buttons that enable tickets to be opened")
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
