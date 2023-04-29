# Trainmarch Tickets
This is a custom ticketing bot for the Trainmarch discord server. It is deployed using Shuttle.

## Ticket Types

* Character Submission
* Respec Submission
* Dungeon Master Application
* Staff Application
* Shopkeep Application
* Sheet Checker Application
* Lore Team Application
* Homebrew Team Application

## Secrets
Secrets are managed using shuttle-secrets. Required fields:
* `DISCORD_TOKEN` The discord bot token.
* `CATEGORY_ID` The category to spawn new tickets in.
* `LOG_CHANNEL` The channel to put the message logs in on ticket closure.
* `SHEETCHECK` The role to ping when opening a new sheetcheck ticket.
* `CHARACTER` The role to ping when opening a character application.
* `HOMEBREW` The role to ping when openign a homebrew team application.
* `SHOPKEEP` The role to ping when opening a shopkeep ticket.
* `BOT_ROLE` The role the bot is assigned.
* `GUILD_ID` The guild ID the bot will be used in.
* `RESPEC` The role to ping when opening a character application.
* `STAFF` The role to ping when opening a Staff applicaiton. This role can see all tickets.
* `LORE` The role to ping when opening a Lore Team application.
* `DM` The role to ping when opening a DM ticket.

## Building and Running

### Prerequisites
You will need to have [protoc](https://github.com/protocolbuffers/protobuf) installed.
You will also need cargo-shuttle installed
```
$ cargo install cargo-shuttle
```
### Local
Running the bot on your local maching can be accomplished by creating and filling out `Secrets.dev.toml` and running shuttle through cargo:
```
$ cargo shuttle run
```
### Remote
Deploying the application to Shuttle follows the standard deployment process. Secrets are stored in `Secrets.toml`.
```
$ cargo shuttle login --api-key YOUR_API_KEY
$ cargo shuttle init
$ cargo shuttle deploy
```

To have the bot run after shuttle's default timeout of 30 minutes, you will need to restart the instance with the updated timeout after deploying the project.
```
$ cargo shuttle project stop
$ cargo shuttle project start --idle-minutes 0
```
