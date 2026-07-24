// • choose a web framework and get familiar with it;
// • define our testing strategy;
// • choose a crate to interact with our database (we will have to save those emails somewhere!);
// • define how we want to manage changes to our database schemas over time (a.k.a. migrations);
// • actually write some queries.
/*
USER STORY
As a blog visitor,
I want to subscribe to the newsletter,
So that I can receive email updates when new content is published on the blog.
*/
//IMPLEMENTing a health check_end point

//web frameworks actix-web,rocket, tide and warp.
//actix-web go to rust frame work for rust production

//receive get request for /health_check and we want to return 200 Ok with no body
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        //server(backbone)
        //app is a builder pattern
        App::new() //component takes incoming request as input and spits out a response
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")? //takes care of where app should be listening
    .run()
    .await
} //HttpServer, in other words, handles all transport level concerns. eg • should we enable transport layer security (TLS)?
