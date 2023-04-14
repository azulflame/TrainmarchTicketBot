use serenity::builder::{CreateActionRow, CreateInputText};

pub mod character;
pub mod dm;
pub mod respec;
pub mod sheetcheck;
pub mod shopkeep;
pub mod staff;

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
}

pub fn build_rows<'a>(
    row: &'a mut CreateActionRow,
    question: &[impl Questions],
) -> &'a mut CreateActionRow {
    question.iter().clone().for_each(|q| {
        row.create_input_text(|t: &mut CreateInputText| {
            t.custom_id(&q.get_id())
                .label(q.get_question())
                .required(q.required())
        });
    });
    row
}
