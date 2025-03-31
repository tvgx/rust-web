use crate::errors::CustomError;
use axum::{response::{Html, Redirect}, Extension};
use axum_extra::extract::Form;
use serde::Deserialize;
use web_pages::root;

pub async fn loader(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users().bind(&client).all().await?;

    let html = root::index(users);

    Ok(Html(html))
}

// 👇 create new SignUp struct
#[derive(Deserialize )]
pub struct SignUp {
    email: String,
}

// 👇 handle form submission
pub async fn new_user_action(
    Extension(pool): Extension<db::Pool>,
    Form(form): Form<SignUp>,
) -> Result<Redirect, CustomError> {
    let client = pool.get().await?;

    let email = form.email;
    let _ = db::queries::users::create_user()
        .bind(&client, &email.as_str())
        .await?;

    // 303 redirect to users list
    Ok(Redirect::to("/"))
}
