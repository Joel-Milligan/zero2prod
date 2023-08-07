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
    <form action="/admin/newsletter" method="post">
        <label>Title
            <input type="text" placeholder="Enter title of newsletter issue" name="title" />
        </label>
        <br/>
        <label>Body
            <input type="text" placeholder="Enter content of newsletter issue" name="content" />
        </label>
        <br/>
        <button type="submit">Publish newsletter</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html"#,
    ))
}
