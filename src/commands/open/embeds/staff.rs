use serenity::builder::CreateEmbed;

pub fn embed(embed: &mut CreateEmbed) -> &mut CreateEmbed {
    embed
    .title("Thanks for the Staff Application")
    .field("A Few Questions", "How old are you?\nHow much experience do you have moderating?\nHow long have you been on the server?\nWhy do you want to become a mod?", false)
    .field("", "Please answer the questons above, then the admins will get back to you when we have the time.", false)
}
