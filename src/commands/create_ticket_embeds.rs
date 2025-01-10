use serenity::all::{CreateActionRow, CreateCommand, CreateEmbed, CreateMessage};
use serenity::builder::CreateButton;
use serenity::client::Context;
use serenity::model::prelude::*;

pub async fn run(ctx: &Context, command: &CommandInteraction) -> Result<String, String> {
    command
        .channel_id
        .send_message(&ctx.http, CreateMessage::new()
            .embed(CreateEmbed::new()
                .title("Character Applications")
                .field("Open an Application", "Click the button below to open a character or respec ticket. Instructions can be found in <#821929650753634314>, or in the messages in the ticket.", false)
            )
            .components( vec![CreateActionRow::Buttons(vec![
                    CreateButton::new("create_character_ticket").label("Create Character").style(ButtonStyle::Primary).to_owned(),
                    CreateButton::new("create_respec_ticket").label("Respec Character").style(ButtonStyle::Primary).to_owned()
                ])]
            )
        )
        .await.map_err(|_| "And error was encountered creating the Character embed".to_string())?;

    command
            .channel_id
            .send_message(&ctx.http, CreateMessage::new()
                    .embed(CreateEmbed::new()
                            .title("Server Applications")
                            .field("Open a Server Application", "These applications are for the roles that aid the server to run. Each ticket will have ", false)
                    )
                .components(vec![
                    CreateActionRow::Buttons( vec![
                        CreateButton::new("create_dm_ticket").label("Dungeon Master").style(ButtonStyle::Primary).to_owned(),
                            CreateButton::new("create_homebrew_ticket").label("Homebrew Team").style(ButtonStyle::Primary).to_owned(),
                            CreateButton::new("create_sheetcheck_ticket").label("Sheet Checker").style(ButtonStyle::Primary).to_owned()
                    ]
                )]
                )
            ).await.map_err(|_|"An error was encountered creating the Server Applications embed".to_string())?;

    command.channel_id.send_message(&ctx.http, CreateMessage::new()
            .embed(CreateEmbed::new()
                                    .title("Server Management Applications")
                    .field("Open an Application", "These tickets are for the roles that manage the server.", false)
                    .field("Instructions", "Each ticket will have instructions and questions inside of the ticket after you open it.", false)
            )
            .components(vec![CreateActionRow::Buttons(
                                    vec![
                                        CreateButton::new("create_staff_ticket").label("Staff").style(ButtonStyle::Primary).to_owned(),
                                            CreateButton::new("create_lore_ticket").label("Lore Team").style(ButtonStyle::Primary).to_owned()
                                    ])
            ])
        ).await.map_err(|_| "And error was encountered creating the Server Management embed".to_string())?;
    command.channel_id.send_message(&ctx.http, CreateMessage::new()
        .embed(CreateEmbed::new()
            .title("Homebrew Submissions")
            .field("Creating a Submission","Here you can submit homebrew items. For the convenience of the homebrew team, they will be opened in a ticket for the homebrew team.", false)
        )
            .components(vec![
                CreateActionRow::Buttons(
                    vec![
                        CreateButton::new("create_homebrew_item").label("Item").style(ButtonStyle::Secondary).to_owned(),
                        CreateButton::new("create_homebrew_spell").label("Spell").style(ButtonStyle::Secondary).to_owned(),
                        CreateButton::new("create_homebrew_subclass").label("Subclass").style(ButtonStyle::Secondary).to_owned(),
                        CreateButton::new("create_homebrew_feat").label("Feat").style(ButtonStyle::Secondary).to_owned(),
                        CreateButton::new("create_homebrew_other").label("Other").style(ButtonStyle::Secondary).to_owned()
                    ]
                )
            ])
    )
        .await.map_err(|_| "An error was encountered creating the Homebrew Submission embed".to_string())?;

    Ok("Embeds Created!".to_string())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("create-ticket-posts")
        .description("Create the embeds and buttons that enable tickets to be opened")
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
