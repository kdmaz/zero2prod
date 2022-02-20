use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use std::fmt::Write;

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();

    for m in flash_messages.iter().filter(|m| m.level() == Level::Error) {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

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
