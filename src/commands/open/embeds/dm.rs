use serenity::builder::CreateEmbed;

pub fn embed(embed: &mut CreateEmbed) -> &mut CreateEmbed {
    embed
        .title("Thanks for the DM Application")
        .field("Please answer these:", "What is your age (optional)?\nHow much experience do you have as a DM?\nHow much time/experience on the server?\nWho on the server can vouch for you?\nWhy do you want to become a DM?", false)
        .field("What Now?", "After you answer the questions, the queen bitch will get back to you. They will review your answers and may go forward with your interview.", false)
}
