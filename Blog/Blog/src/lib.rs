use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
//HttpServer is the top-level struct that manages the actual TCP listener, worker threads, and the lifecycle of your web server.
use std::net::TcpListener;
//Initially HttpServer was doing double duty: given an address, it will bind it and then start the application. W
//Using TcpListener we will bind the port on our own with TcpListener and then hand that over to the HttpServer using listen.
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}
//A handler is just a regular function (often async) that actix-web calls when an incoming request matches a particular route.

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
//instead of waiting for server we return server future as the positive side of the result enum
pub fn run(listener: TcpListener, // address: &str
) -> Result<Server, std::io::Error> {
    //builds the server
    //future represents “keep listening for requests forever.”
    let server = HttpServer::new(|| {
        //HttpServer::new(factory) takes a closure — not an App instance directly — because HttpServer spins up multiple worker threads (one per CPU core, by default), and each worker needs its own separate App instance.
        App::new()
            .route("/health_check", web::get().to(health_check))
            //.to() is a method on Route whose job is: "take this handler function, and remember it as the thing to call whenever this route matches an incoming request."
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    //  .bind(address)? //"127.0.0.1:8000")?
    .run();
    // No .await here!
    Ok(server)
}
//: tests should run their background application on a random available port.
//so we change gard coded address to avilable address
//: trying to bind port 0 will trigger an OS scan for an available port which will then be bound to the application

//How Actix-Web Automates Parsing
/*/------------------
Parsing is the process of taking raw, unstructured data (like a string of text) and analyzing it to extract meaningful, structured information that a program can easily work with.

Think of it like reading a sentence:

Raw Input: "The big red car"

Parsing: Your brain breaks this down into components—"The" (article), "big red" (adjectives), "car" (noun).

Structured Output: You now understand the role each word plays, rather than just seeing a sequence of characters.
//-----------------*/

//Normally, when a web server receives an HTTP POST request containing form data, you would have to manually read the raw request bytes, parse the URL-encoded string, and map those fields into a data structure.
//A URL-encoded string (also called percent-encoding) is a special format used to turn characters that are "unsafe" or special in web addresses into a format that web servers can read reliably.
/*Why URL Encoding Exists
URLs (web links) can only safely contain a limited set of ASCII characters (alphanumeric characters like A-Z, a-z, 0-9, and a few symbols like -, _, .).

If you try to put spaces, special symbols (like ?, &, #, /), or non-English characters (like é or 🔥) in a URL or form data, it will break the link or confuse the web server.

To fix this, web browsers convert those "unsafe" characters into a % followed by a two-character hexadecimal number representing their ASCII/UTF-8 code. */

/*
Phase 1: The Endpoint is "Empty" (Before)
At first, the book has you write a minimal subscribe handler:

Rust
async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}
What it does: It receives an incoming request and immediately sends back a 200 OK status code.

The Problem: It is completely "deaf." If a user sends their name or email address in a form, this function doesn't read it, process it, or save it. It just ignores everything sent to it.

Phase 2: The Lesson (The "Side Example")
Before fixing that "deaf" function, the author stops to teach you a new concept: How to read form data in Actix-web.

The author introduces this example to demonstrate the pattern:

Rust
fn index(form: web::Form<FormData>) -> String {
    format!("Welcome {}!", form.username)
}
What the author is teaching: By simply adding form: web::Form<FormData> into the function's arguments, Actix-web will automatically intercept the incoming request, parse the URL-encoded string, and hand you a clean Rust struct (form.username).

Phase 3: The Update (After)
Now that you know how web::Form works from that lesson, the book tells you to go back and update your subscribe function so it can actually handle user sign-ups:

Rust
// 1. You define what data you want to extract
#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

// 2. You UPDATE subscribe using the trick you just learned
async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    println!("New subscriber: {} ({})", form.name, form.email);
    HttpResponse::Ok().finish()
}
*/
