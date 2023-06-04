use actix_web::HttpResponse;

#[tracing::instrument(name = "Health check called")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
