use serenity::{
    builder::{CreateComponents, CreateEmbed, CreateInteractionResponse},
    model::prelude::interaction::InteractionResponseType,
};

pub fn embed(embed: &mut CreateEmbed) -> &mut CreateEmbed {
    embed
    .title("Thanks for the Staff Application")
    .field("A Few Questions", "How old are you?\nHow much experience do you have moderating?\nHow long have you been on the server?\nWhy do you want to become a mod?", false)
    .field("", "Please answer the questons above, then the admins will get back to you when we have the time.", false)
}
pub enum StaffQuestions {
    Age,
    Experience,
    ServerTime,
    Why,
}

impl super::Questions for StaffQuestions {
    fn get_question(&self) -> &str {
        match &self {
            StaffQuestions::Age => "What is your age?",
            StaffQuestions::Experience => "How much experience do you have moderating?",
            StaffQuestions::ServerTime => "How long have you been on the server?",
            StaffQuestions::Why => "Why do you want to be a mod?",
        }
    }
    fn get_id(&self) -> &str {
        match &self {
            StaffQuestions::Age => "staff_modal_age",
            StaffQuestions::Experience => "staff_modal_experience",
            StaffQuestions::ServerTime => "staff_modal_servertime",
            StaffQuestions::Why => "staff_modal_why",
        }
    }
    fn required(&self) -> bool {
        match self {
            StaffQuestions::Age => true,
            StaffQuestions::Experience => true,
            StaffQuestions::ServerTime => true,
            StaffQuestions::Why => true,
        }
    }
}

const STAFF_QUESTIONS: [StaffQuestions; 4] = [
    StaffQuestions::Age,
    StaffQuestions::Experience,
    StaffQuestions::ServerTime,
    StaffQuestions::Why,
];

pub fn get_modal<'a>(
    z: &'a mut CreateInteractionResponse<'a>,
) -> &'a mut CreateInteractionResponse<'a> {
    z.kind(InteractionResponseType::Modal)
        .interaction_response_data(|f| {
            f.custom_id("staff_modal_submit")
                .components(|c: &mut CreateComponents| {
                    c.create_action_row(|row| super::build_rows(row, &STAFF_QUESTIONS))
                })
        })
}
