use actix_web::{HttpResponse, Responder};

pub async fn list_models() -> impl Responder {
    // Placeholder for listing models. Will eventually proxy to Gemini API.
    HttpResponse::Ok().json(vec!["model1", "model2"])
}
