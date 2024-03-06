use actix_web::{http::header::ContentType, HttpResponse};

pub async fn publish_newsletter_form() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>New issue</title>
</head>
<body>
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
        <button type="submit">Publish new issue</button>
    </form>
    <p><a href="/admin/dashboard>&lt;- Back</a></p>
</body>
</html>"#,
    ))
}
