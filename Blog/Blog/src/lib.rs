use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
//instead of waiting for server we return server future as the possitive side of the result enum
pub fn run() -> Result<Server, std::io::Error> {
    //builds the server
    //future represents “keep listening for requests forever.”
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();
    // No .await here!
    Ok(server)
}
