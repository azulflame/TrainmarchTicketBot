mod commands;
mod config;

use commands::open::TicketType;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseFollowup, GuildId, Interaction, Ready};
use serenity::prelude::*;
use serenity::{async_trait, Client};
use shuttle_runtime::SecretStore;
use std::error::Error;
use std::str::FromStr;
use crate::commands::open::open_modal;
struct Handler;
#[derive(Eq, PartialEq)]
enum CommandType {
    CreateTicket,
    Close,
    HomebrewClose,
}
impl FromStr for CommandType {
    type Err = ();
    fn from_str(input: &str) -> Result<CommandType, ()> {
        match input {
            "close" => Ok(CommandType::Close),
            "create-ticket-posts" => Ok(CommandType::CreateTicket),
            "homebrew-close" => Ok(CommandType::HomebrewClose),
            _ => Err(()),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    // When the bot is ready
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let guild_id_num = config::get_config_val(config::SecretType::GuildId)
            .parse()
            .expect("GUILD_ID must be an integer");
        let guild_id = GuildId::new(guild_id_num);

        let _commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::close::register(),
                commands::create_ticket_embeds::register(),
                commands::close_homebrew::register(),
            ]
            )
            .await
            .unwrap();
        println!("Bot ready!");
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // message button interaction
        if let Interaction::Component(command) = interaction.clone() {
            if let Ok(ticket_type) = TicketType::from_str(&command.data.custom_id) {
                println!("Create Interaction: {}", &command.data.custom_id.to_string());
                let response = match ticket_type {
                    // Directly open a ticket
                    TicketType::Character => {
                        command.defer(&ctx.http).await.unwrap();
                        commands::open::run(&ctx, &command, TicketType::Character).await
                    }
                    TicketType::Respec => {
                        command.defer(&ctx.http).await.unwrap();
                        commands::open::run(&ctx, &command, TicketType::Respec).await
                    }
                    // Open a modal
                    TicketType::Dm
                    | TicketType::Sheetcheck
                    | TicketType::Homebrew
                    | TicketType::Staff
                    | TicketType::Lore
                    | TicketType::HbSubclass
                    | TicketType::HbFeat
                    | TicketType::HbItem
                    | TicketType::HbSpell
                    | TicketType::HbOther => open_modal(&ctx, &command, ticket_type).await,
                };
                if ticket_type == TicketType::Character || ticket_type == TicketType::Respec {
                    match match response {
                        Ok(x) => {
                            command
                                .create_followup(&ctx.http,
                                                 CreateInteractionResponseFollowup::new()
                                                     .content(format!("Your ticket has been created at <#{}>", x))
                                                     .ephemeral(true)
                                )
                                .await
                        }
                        Err(x) => {
                            command
                                .create_followup(&ctx.http, CreateInteractionResponseFollowup::new()
                                    .content(format!(
                                        "There was an error opening your ticket: {}",
                                        x
                                    ))
                                    .ephemeral(true)
                                )
                                .await
                        }
                    } {
                        Ok(_) => {}
                        Err(e) => println!("{:#?}", e),
                    }
                }
            }
        }
        // Slash command interaction
        if let Interaction::Command(command) = interaction.clone() {
            command.defer(&ctx.http).await.unwrap();
            if let Ok(command_type) = CommandType::from_str(&command.data.name) {
                let response = match command_type {
                    CommandType::Close => commands::close::run(&ctx, &command).await,
                    CommandType::CreateTicket => {
                        commands::create_ticket_embeds::run(&ctx, &command).await
                    },
                    CommandType::HomebrewClose => {commands::close_homebrew::run(&ctx, &command).await},
                };

                let _x = match response {
                    Ok(_) if command_type != CommandType::Close => command
                        .create_followup(&ctx.http, CreateInteractionResponseFollowup::new()
                            .content("Created").ephemeral(true))
                        .await
                        .is_ok(),
                    Ok(_) => command
                        .create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(Default::default()))
                        .await
                        .is_ok(),

                    Err(x) => command
                        .create_followup(&ctx.http, CreateInteractionResponseFollowup::new().content(format!("There was an error opening your ticket: {}", x))
                                .ephemeral(true)
                        )
                        .await
                        .is_ok()
                };
            }
        }

        // Modal submission
        if let Interaction::Modal(submission) = interaction.clone() {
            submission.defer(&ctx.http).await.unwrap();
            println!("{}", submission.data.custom_id);
            let message = match match TicketType::from_str(submission.data.custom_id.as_str()) {
                Err(_) => Err(
                    "Unable to identify which ticket was used. Please contact Azulflame".to_owned(),
                ),
                Ok(ticket_type) => {
                    commands::open::create_ticket_from_modal(&ctx, &submission, ticket_type).await
                }
            } {
                Ok(x) => format!("Your ticket has been opened at <#{}>", x),

                Err(x) => format!("There was an error opening your ticket: {}", x),
            };
            submission
                .create_followup(&ctx.http, CreateInteractionResponseFollowup::new().content(message).ephemeral(true))
                .await
                .unwrap();
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    config::load_config(secrets);
    let token = config::get_config_val(config::SecretType::DiscordToken);
    let client = Client::builder(token, GatewayIntents::MESSAGE_CONTENT)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    Ok(client.into())
}
