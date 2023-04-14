use serenity::{
    builder::{CreateComponents, CreateEmbed, CreateInteractionResponse},
    model::prelude::interaction::InteractionResponseType,
};

pub fn embed(embed: &mut CreateEmbed) -> &mut CreateEmbed {
    embed
        .title("Thanks for the DM Application")
        .field("Please answer these:", "What is your age (optional)?\nHow much experience do you have as a DM?\nHow much time/experience on the server?\nWho on the server can vouch for you?\nWhy do you want to become a DM?", false)
        .field("What Now?", "After you answer the questions, the queen bitch will get back to you. They will review your answers and may go forward with your interview.", false)
}

pub enum DmQuestions {
    Age,
    Experience,
    ServerTime,
    Vouch,
    Why,
}

impl super::Questions for DmQuestions {
    fn get_question(&self) -> &str {
        match &self {
            DmQuestions::Age => "What is your age?",
            DmQuestions::Experience => "How much experience do you have as a DM?",
            DmQuestions::ServerTime => "How long have you been on the server?",
            DmQuestions::Vouch => "Who on the server can vouch for you?",
            DmQuestions::Why => "Why do you want to be a DM?",
        }
    }
    fn get_id(&self) -> &str {
        match &self {
            DmQuestions::Age => "dm_modal_age",
            DmQuestions::Experience => "dm_modal_experience",
            DmQuestions::ServerTime => "dm_modal_servertime",
            DmQuestions::Vouch => "dm_modal_vouch",
            DmQuestions::Why => "dm_modal_why",
        }
    }
    fn required(&self) -> bool {
        match &self {
            DmQuestions::Age => false,
            DmQuestions::Experience => true,
            DmQuestions::ServerTime => true,
            DmQuestions::Vouch => true,
            DmQuestions::Why => true,
        }
    }
}
static DM_QUESTIONS: [DmQuestions; 5] = [
    DmQuestions::Age,
    DmQuestions::Experience,
    DmQuestions::ServerTime,
    DmQuestions::Vouch,
    DmQuestions::Why,
];

pub fn get_modal<'a>(
    z: &'a mut CreateInteractionResponse<'a>,
) -> &'a mut CreateInteractionResponse<'a> {
    z.kind(InteractionResponseType::Modal)
        .interaction_response_data(|f| {
            f.custom_id("dm_modal_submit")
                .components(|c: &mut CreateComponents| {
                    c.create_action_row(|r| super::build_rows(r, &DM_QUESTIONS))
                })
        })
}
