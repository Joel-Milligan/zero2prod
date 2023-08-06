use actix_web::{http::header::ContentType, HttpResponse};

pub async fn publish_newsletter_form() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content=type" content="text/html; charset=utf-8" />
    <title>Send Newsletter</title>
</head>
<body>
    <form action="/admin/password" method="post">
        <label>Current password
            <input type="password" placeholder="Enter current password" name="current_password" />
        </label>
        <br/>
        <label>New password
            <input type="password" placeholder="Enter new password" name="new_password" />
        </label>
        <br/>
        <label>Confirm new password
            <input type="password" placeholder="Enter new password again" name="new_password_check" />
        </label>
        <br/>
        <button type="submit">Change password</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html"#
    ))
}