use chrono::format::Item;
use self::{
    dm::DmQuestions, homebrew::HomebrewQuestions, lore::LoreQuestions,
    sheetcheck::SheetcheckQuestions, staff::StaffQuestions,
};
use serenity::all::{CreateActionRow, CreateInputText, CreateInteractionResponse, CreateModal, InputTextStyle};
use serenity::{model::prelude::*, prelude::Context};
use crate::commands::tickets::hb_feat::FeatQuestions;
use crate::commands::tickets::hb_item::ItemQuestions;
use crate::commands::tickets::hb_other::OtherQuestions;
use crate::commands::tickets::hb_spell::SpellQuestions;
use crate::commands::tickets::hb_subclass::SubclassQuestions;

pub mod character;
pub mod dm;
pub mod homebrew;
pub mod lore;
pub mod respec;
pub mod sheetcheck;
pub mod staff;
pub mod hb_item;
pub mod hb_spell;
pub mod hb_subclass;
pub mod hb_feat;
pub mod hb_other;

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
    interaction: &ComponentInteraction,
    questions: &Vec<Box<impl Questions + ?Sized>>,
    title: String,
) {
    let x = interaction
        .create_response(&ctx.http,
                         CreateInteractionResponse::Modal(CreateModal::new(id, title)
                             .components(
                                     questions
                                         .iter()
                                         .map(|x| {
                                            CreateActionRow::InputText(CreateInputText::new(x.style(), x.get_question(), x.get_id()).required(x.required()).max_length(800))
                                         })
                                         .collect::<Vec<CreateActionRow>>()
                                 )
                         )
        )
        .await;
    match x {
        Ok(_) => {}
        Err(y) => {
            println!("{:#?}", y);
        }
    }
}
pub fn get_question_from_id(id: &str) -> String {
    println!("Question ID: {}", &id);
    let question = match id {
        _ if id == DmQuestions::Age.get_id() => DmQuestions::Age.get_question(),
        _ if id == DmQuestions::Experience.get_id() => DmQuestions::Experience.get_question(),
        _ if id == DmQuestions::Vouch.get_id() => DmQuestions::Vouch.get_question(),
        _ if id == DmQuestions::Why.get_id() => DmQuestions::Why.get_question(),
        _ if id == DmQuestions::ServerTime.get_id() => DmQuestions::ServerTime.get_question(),
        _ if id == SheetcheckQuestions::Age.get_id() => SheetcheckQuestions::Age.get_question(),
        _ if id == SheetcheckQuestions::ServerTime.get_id() => SheetcheckQuestions::ServerTime.get_question(),
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
        _ if id == ItemQuestions::Rarity.get_id() => ItemQuestions::Rarity.get_question(),
        _ if id == ItemQuestions::Link.get_id() => ItemQuestions::Link.get_question(),
        _ if id == ItemQuestions::Name.get_id() => ItemQuestions::Name.get_question(),
        _ if id == ItemQuestions::Description.get_id() => ItemQuestions::Description.get_question(),
        _ if id == FeatQuestions::Description.get_id() => FeatQuestions::Description.get_question(),
        _ if id == FeatQuestions::Name.get_id() => FeatQuestions::Name.get_question(),
        _ if id == FeatQuestions::Link.get_id() => FeatQuestions::Link.get_question(),
        _ if id == SpellQuestions::Link.get_id() => SpellQuestions::Link.get_question(),
        _ if id == SpellQuestions::Name.get_id() => SpellQuestions::Name.get_question(),
        _ if id == SpellQuestions::Description.get_id() => SpellQuestions::Description.get_question(),
        _ if id == SpellQuestions::Level.get_id() => SpellQuestions::Level.get_question(),
        _ if id == SubclassQuestions::Description.get_id() => SubclassQuestions::Description.get_question(),
        _ if id == SubclassQuestions::Name.get_id() => SubclassQuestions::Name.get_question(),
        _ if id == SubclassQuestions::Link.get_id() => SubclassQuestions::Link.get_question(),
        _ if id == SubclassQuestions::Class.get_id() => SubclassQuestions::Class.get_question(),
        _ if id == OtherQuestions::Link.get_id() => OtherQuestions::Link.get_question(),
        _ if id == OtherQuestions::Name.get_id() => OtherQuestions::Name.get_question(),
        _ if id == OtherQuestions::Description.get_id() => OtherQuestions::Description.get_question(),
        x => {println!("PANIC: {}", x); panic!();}
    };
    question.to_owned().to_string()
}
