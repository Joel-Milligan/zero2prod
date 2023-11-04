use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn publish_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    let idempotency_key = uuid::Uuid::new_v4();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content=type" content="text/html; charset=utf-8" />
    <title>Send Newsletter</title>
</head>
<body>
    {msg_html}
    <form action="/admin/newsletter" method="post">
        <label>Title
            <input type="text" placeholder="Enter title of newsletter issue" name="title" />
        </label>
        <br/>
        <label>Text
            <input type="text" placeholder="Enter content of newsletter issue" name="text_content" />
        </label>
        <br/>
        <label>HTML
            <input type="text" placeholder="Enter HTML of newsletter issue" name="html_content" />
        </label>
        <br/>
        <input hidden type="text" name="idempotency_key" value="{idempotency_key}" />
        <button type="submit">Publish newsletter</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html"#
        )))
}
