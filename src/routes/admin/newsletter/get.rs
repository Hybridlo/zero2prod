use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write as _;

pub async fn publish_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    let idempotency_key = uuid::Uuid::new_v4().to_string();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>New issue</title>
</head>
<body>
    {msg_html}
    <form action="/admin/newsletter" method="post">
        <label>
            New issue title
            <input
                type="text"
                placeholder="Enter new issue title"
                name="title"
            >
        </label>
        <br>
        <label>
            New issue text content
            <input
                type="text"
                placeholder="Enter new issue text content"
                name="text_content"
            >
        </label>
        <br>
        <label>
            New issue html content
            <input
                type="text"
                placeholder="Enter new issue html content"
                name="html_content"
            >
        </label>
        <br>
        <input hidden type="text" name="idempotency_key" value="{idempotency_key}"
        <button type="submit">Publish new issue</button>
    </form>
    <p><a href="/admin/dashboard>&lt;- Back</a></p>
</body>
</html>"#,
        )))
}
