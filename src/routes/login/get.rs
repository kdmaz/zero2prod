use actix_web::{cookie::Cookie, http::header::ContentType, web, HttpResponse};

pub async fn login_form(request: web::HttpRequest) -> HttpResponse {
    let error_html: String = match request.cookie("_flash") {
        None => "".into(),
        Some(cookie) => {
            format!("<p><i>{}</i></p>", cookie.value())
        }
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(
            Cookie::build("_flash", "")
                .max_age(time::Duration::ZERO)
                .finish(),
        )
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
