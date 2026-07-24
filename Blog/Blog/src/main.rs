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
    //asynchronous based on future trait
    //main is asynchronous coz HttpServer::run is an asynchronous method
    HttpServer::new(|| {
        //HttpServer::run() starts listening for incoming connections.
        //server(backbone)
        //app is a builder pattern
        App::new() //component takes incoming request as input and spits out a response
            .route("/", web::get().to(greet))
            //route method helps us add new end point to our app
            .route(
                "/{name}",
                web::get() //the request should bepassed to the handler if and only if its HTTP method is GET. You can start to picture what happens when a new request come
                    .to(greet),
            )
    })
    .bind("127.0.0.1:8000")? //takes care of where app should be listening
    .run()
    .await
} //HttpServer, in other words, handles all transport level concerns. eg • should we enable transport layer security (TLS)?

/*async fn greet(req: HttpRequest) -> impl Responder {
[...]
}
  greet is an asynchronous function that takes an HttpRequest as input and returns something that implements the Responder trait1
 */
//A type implements the Responder trait if it can be converted into a HttpResponse
/*An endpoint is basically a rule that says:

“When a request comes in at this path, with these conditions, run this handler.”

You add endpoints to your app using .route().

.route(path, route)
path: The URL pattern (like "/", "/users", or "/{id}" for dynamic segments).

route: A Route object that combines:

A handler (the function that runs when matched).

guards (conditions like HTTP method, headers, etc.).

🔒 Guards
Guards are filters. They decide if a request qualifies for that route.
Example: web::get() is shorthand for “only match if the request method is GET.”

So this line:

rust
.route("/", web::get().to(greet))
means:

Path must be / (no extra segments).

Method must be GET.

If both match, call the greet handler. */
