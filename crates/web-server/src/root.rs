use axum::{
    Extension,
    response::{Html, Redirect},
};
use axum_extra::extract::Form;
use serde::Deserialize;
use web_pages::root;

use crate::errors::CustomError;

pub async fn loader(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;
    let users = db::queries::users::get_users().bind(&client).all().await?;
    let html = root::index(users);
    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct SignUp {
    email: String,
}

pub async fn new_user_action(
    Extension(pool): Extension<db::Pool>,
    Form(form): Form<SignUp>,
) -> Result<Redirect, CustomError> {
    let client = pool.get().await?;
    let email = form.email;
    let _ = db::queries::users::create_user()
        .bind(&client, &email.as_str())
        .await?;
    Ok(Redirect::to("/"))
}
