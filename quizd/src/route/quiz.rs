use crate::QuizdError;
use handlebars::Handlebars;
use rocket::State;

#[rocket::get("/q/<shortcode>")]
pub fn get(
    shortcode: String,
    template_engine: State<Handlebars<'static>>,
) -> Result<(), QuizdError> {
    todo!()
}
