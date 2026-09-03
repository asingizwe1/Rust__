use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;
//Initially HttpServer was doing double duty: given an address, it will bind it and then start the application. W
//Using TcpListener we will bind the port on our own with TcpListener and then hand that over to the HttpServer using listen.

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
//instead of waiting for server we return server future as the possitive side of the result enum
pub fn run(listener: TcpListener, // address: &str
) -> Result<Server, std::io::Error> {
    //builds the server
    //future represents “keep listening for requests forever.”
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        //  .bind(address)? //"127.0.0.1:8000")?
        .run();
    // No .await here!
    Ok(server)
}
//: tests should run their background application on a random available port.
//so we change gard coded address to avilable address
//: trying to bind port 0 will trigger an OS scan for an available port which will then be bound to the application
