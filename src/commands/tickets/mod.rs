use serenity::{
    model::prelude::{
        component::InputTextStyle,
        interaction::{message_component::MessageComponentInteraction, InteractionResponseType},
    },
    prelude::Context,
};

use self::{
    dm::DmQuestions, lore::LoreQuestions, sheetcheck::SheetcheckQuestions,
    shopkeep::ShopkeepQuestions, staff::StaffQuestions, homebrew::HomebrewQuestions,
};

pub mod character;
pub mod dm;
pub mod lore;
pub mod respec;
pub mod sheetcheck;
pub mod shopkeep;
pub mod staff;
pub mod homebrew;

pub trait Questions {
    fn get_question(&self) -> &str {
        unimplemented!()
    }
    fn get_id(&self) -> &str {
        unimplemented!()
    }
    fn required(&self) -> bool {
        unimplemented!();
    }
    fn style(&self) -> InputTextStyle {
        unimplemented!();
    }
}


pub async fn send_modal(
    id: String,
    ctx: &Context,
    interaction: &MessageComponentInteraction,
    questions: &Vec<Box<impl Questions + ? Sized>>,
    title: String,
) {
    let x = interaction
        .create_interaction_response(&ctx.http, |f| {
            f.kind(InteractionResponseType::Modal)
                .interaction_response_data(|d| {
                    d.title(title).custom_id(id).components(|c| {
                        questions.iter().clone().fold(c, |c, val| {
                            c.create_action_row(|row| {
                                row.create_input_text(|t| {
                                    t.custom_id((*val).get_id())
                                        .max_length(800)
                                        .label((*val).get_question())
                                        .required((*val).required())
                                        .style((*val).style())
                                })
                            })
                        })
                    })
                })
        })
        .await;
    match x {
        Ok(_) => {}
        Err(y) => {
            println!("{:#?}", y);
        }
    }
}
pub fn get_question_from_id(id: &str) -> String {
    let question = match id {
        _ if id == ShopkeepQuestions::Shop.get_id() => ShopkeepQuestions::Shop.get_question(),
        _ if id == ShopkeepQuestions::Why.get_id() => ShopkeepQuestions::Why.get_question(),
        _ if id == ShopkeepQuestions::Character.get_id() => {
            ShopkeepQuestions::Character.get_question()
        }
        _ if id == DmQuestions::Age.get_id() => DmQuestions::Age.get_question(),
        _ if id == DmQuestions::Experience.get_id() => DmQuestions::Experience.get_question(),
        _ if id == DmQuestions::Vouch.get_id() => DmQuestions::Vouch.get_question(),
        _ if id == DmQuestions::Why.get_id() => DmQuestions::Why.get_question(),
        _ if id == DmQuestions::ServerTime.get_id() => DmQuestions::ServerTime.get_question(),
        _ if id == SheetcheckQuestions::Age.get_id() => SheetcheckQuestions::Age.get_question(),
        _ if id == SheetcheckQuestions::ServerTime.get_id() => {
            SheetcheckQuestions::ServerTime.get_question()
        }
        _ if id == SheetcheckQuestions::Why.get_id() => SheetcheckQuestions::Why.get_question(),
        _ if id == StaffQuestions::Age.get_id() => StaffQuestions::Age.get_question(),
        _ if id == StaffQuestions::Experience.get_id() => StaffQuestions::Experience.get_question(),
        _ if id == StaffQuestions::Why.get_id() => StaffQuestions::Why.get_question(),
        _ if id == StaffQuestions::ServerTime.get_id() => StaffQuestions::ServerTime.get_question(),
        _ if id == LoreQuestions::Age.get_id() => LoreQuestions::Age.get_question(),
        _ if id == LoreQuestions::ServerTime.get_id() => LoreQuestions::ServerTime.get_question(),
        _ if id == LoreQuestions::Vouch.get_id() => LoreQuestions::Vouch.get_question(),
        _ if id == LoreQuestions::Why.get_id() => LoreQuestions::Why.get_question(),
        _ if id == HomebrewQuestions::Why.get_id() => HomebrewQuestions::Why.get_question(),
        _ if id == HomebrewQuestions::Experience.get_id() => HomebrewQuestions::Experience.get_question(),
        _ if id == HomebrewQuestions::Balance.get_id() => HomebrewQuestions::Balance.get_question(),
        _ if id == HomebrewQuestions::Time.get_id() => HomebrewQuestions::Time.get_question(),
        _ => panic!(),
    };
    question.to_owned().to_string()
}
