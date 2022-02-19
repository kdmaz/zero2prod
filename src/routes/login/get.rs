use actix_web::{http::header::ContentType, web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let error_html = match query.0.error {
        None => "".into(),
        Some(error_message) => format!("<p><i>{}</i></p>", error_message),
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            
            <head>
                <meta charset="UTF-8">
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Login</title>
            </head>
            
            <body>
                <form action="/login" method="post">
                    <div>{}</div>
                    <div>
                        <label for="username">Username</label>
                        <input type="text" name="username" autocomplete="username"
                            placeholder="Enter Username">
                    </div>
            
                    <div>
                        <label for="password">Password</label>
                        <input type="password" name="password"
                            autocomplete="current-password" placeholder="Enter Password">
                    </div>
            
                    <button type="submit">Login</button>
                </form>
            </body>
            
            </html>"#,
            error_html
        ))
}
