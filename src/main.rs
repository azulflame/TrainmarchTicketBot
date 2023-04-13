mod commands;
mod config;
use std::str::FromStr;

use serenity::{
    async_trait,
    model::prelude::{interaction::Interaction, GuildId, Ready},
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};

use commands::open::TicketType;
use shuttle_secrets::SecretStore;
struct Handler;
#[derive(Eq, PartialEq)]
enum CommandType {
    CreateTicket,
    Close,
}
impl FromStr for CommandType {
    type Err = ();
    fn from_str(input: &str) -> Result<CommandType, ()> {
        match input {
            "close" => Ok(CommandType::Close),
            "create-ticket-posts" => Ok(CommandType::CreateTicket),
            _ => Err(()),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    // Handle slash commands or other interactions
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // Message button interaction
        if let Interaction::MessageComponent(command) = interaction.clone() {
            command.defer(&ctx.http).await.unwrap();
            if let Ok(ticket_type) = TicketType::from_str(&command.data.custom_id) {
                println!("ticket type: {:#?}", ticket_type);
                let response = match ticket_type {
                    TicketType::Character => {
                        commands::open::run(&ctx, &command, TicketType::Character).await
                    }
                    TicketType::Dm => commands::open::run(&ctx, &command, TicketType::Dm).await,
                    TicketType::Respec => {
                        commands::open::run(&ctx, &command, TicketType::Respec).await
                    }
                    TicketType::Shopkeep => {
                        commands::open::run(&ctx, &command, TicketType::Shopkeep).await
                    }
                    TicketType::Sheetcheck => {
                        commands::open::run(&ctx, &command, TicketType::Sheetcheck).await
                    }
                    TicketType::Staff => {
                        commands::open::run(&ctx, &command, TicketType::Staff).await
                    }
                };
                match match response {
                    Ok(x) => {
                        command
                            .create_followup_message(&ctx.http, |f| {
                                f.content(format!("Your ticket has been created at <#{}>", x))
                                    .ephemeral(true)
                            })
                            .await
                    }
                    Err(x) => {
                        command
                            .create_followup_message(&ctx.http, |f| {
                                f.content(format!("There was an error opening your ticket: {}", x))
                                    .ephemeral(true)
                            })
                            .await
                    }
                } {
                    Ok(_) => {}
                    Err(e) => println!("{:#?}", e),
                }
            }
        }

        // Slash command interaction
        if let Interaction::ApplicationCommand(command) = interaction.clone() {
            command.defer(&ctx.http).await.unwrap();
            if let Ok(command_type) = CommandType::from_str(&command.data.name) {
                let response = match command_type {
                    CommandType::Close => commands::close::run(&ctx, &command).await,
                    CommandType::CreateTicket => {
                        commands::create_ticket_embeds::run(&ctx, &command).await
                    }
                };

                let _x = match response {
                    Ok(_) if command_type != CommandType::Close => {
                        command
                            .create_followup_message(&ctx.http, |f| {
                                f.content("Your ticket has been closed").ephemeral(true)
                            })
                            .await.is_ok()
                    }
                    Ok(_) => {
                        command.create_interaction_response(&ctx.http, |f| f.kind(serenity::model::prelude::interaction::InteractionResponseType::DeferredUpdateMessage)).await.is_ok()
                    }
                    Err(x) => {
                        command
                            .create_followup_message(&ctx.http, |f| {
                                f.content(format!("There was an error opening your ticket: {}", x))
                                    .ephemeral(true)
                            })
                            .await.is_ok()
                    }
                };
            }
        }
    }

    // When the bot is ready
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let guild_id_num = config::get_config_val(config::SecretType::GuildId)
            .parse()
            .expect("GUILD_ID must be an integer");
        let guild_id = GuildId(guild_id_num);

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::close::register(command))
                .create_application_command(|command| {
                    commands::create_ticket_embeds::register(command)
                })
        })
        .await
        .unwrap();
        println!("Bot ready!");
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    config::load_config(secrets);
    let token = config::get_config_val(config::SecretType::DiscordToken);
    let client = Client::builder(token, GatewayIntents::MESSAGE_CONTENT)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    Ok(client.into())
}
