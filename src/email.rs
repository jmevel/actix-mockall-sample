use actix_web::{web::Data, HttpResponse};

use crate::email_client::EmailClient;

pub async fn email_test(email_client: Data<Box<dyn EmailClient>>) -> HttpResponse {
    match email_client.send_email(
        "Bob <bob@domain.com>",
        "This is a test email",
        "Text content",
    ) {
        Ok(_) => return HttpResponse::Ok().body("Email Sent"),
        Err(e) => return HttpResponse::InternalServerError().body(e),
    };
}
