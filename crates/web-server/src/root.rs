use axum::{Extension, response::Html};
use web_pages::root;

use crate::errors::CustomError;

pub async fn loader(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;
    let users = db::queries::users::get_users().bind(&client).all().await?;
    let html = root::index(users);
    Ok(Html(html))
}
