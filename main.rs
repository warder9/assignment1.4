mod api;
use actix_web::{web, App, HttpServer};
use api::search_news;
use actix_web::{HttpResponse, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Crypto News Aggregator is running! Try /search/bitcoin")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok(); 

    println!("🚀 Server running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
			.route("/", web::get().to(index))
            .route("/search/{query}", web::get().to(search_news))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
