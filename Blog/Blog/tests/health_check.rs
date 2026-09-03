//! tests/health_check.rs
// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test] //unit tests are #[cfg(test)]
               //In Rust, every project is a crate. Your project is named zero2prod, so that’s the crate root.
               //When you run integration tests (those inside the tests/ directory), they are compiled as separate crates.-> That means they don’t automatically have access to your project’s internal modules.
               //To use your project code, you import it just like any other external dependency: use zero2prod::something.
               // async fn spawn_app() -> std::io::Result<()> {
               //     zero2prod::run().await //package name in unit test
               // }
async fn health_check_works() {
    // Arrange  -> spawn_app piece is the only piece that will depend on our application code
    // spawn_app().await.expect("Failed to spawn our app.");
    let address = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &address))
        //.get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
// Launch our application in the background ~somehow~
// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
fn spawn_app() {
    //Your test future continues immediately — so you can send requests and check responses.
    //No async spawn_app → because awaiting the server would block forever.

    //HttpServer::run() gives you a Server future that never resolves — it just keeps listening for requests.
    //.await it inside your test setup (spawn_app), the test runtime gets stuck forever and your assertions never run.
    // No .await, no .expect
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    //bind ->Creates a socket: It asks the operating system for a TCP socket.,Associates it with an address,Reserves the port: Once bound, the socket is ready to listen for incoming connections.

    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    ////: trying to bind port 0 will trigger an OS scan for an available port which will then be bound to the application
    let server = zero2prod::run(
        listener, //    "127.0.0.1:0" - this would cause issues so we resortrf to listen
    )
    .expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server); // runs the server in the background, so your test logic continues.
                                  /*tokio::spawn takes a future and tells the Tokio runtime:

                                                                    “Run this future in the background, alongside other futures.”

                                                                    It does not create a new OS thread. Instead, it schedules the future on Tokio’s async task system (like lightweight green threads).
                                                                    //UNDERSTANDING
                                                                     tests:

                                  You spawn the server future.

                                  The server starts listening.

                                  Your test logic can continue: send HTTP requests, check responses, finish.

                                  If you had .awaited the server in a test, the runtime would never move past that line — your assertions would never run.
                                                                    */
    format!("http://127.0.0.1:{}", port)
}

//we restrefactor our project into library and a binary: : all our logic will live in the library crate while the binary itself will be just an entrypoint with a very slim main function.
// library crate (for reusable logic) and a binary crate (for the executable entrypoint).
/*Library crate (lib.rs) → contains reusable logic, functions, structs, modules.

 moving all your business logic, algorithms, and reusable components into the library crate, leaving the binary crate as a thin wrapper that just calls into the library.

Binary crate (main.rs) → contains the entrypoint (fn main()), typically minimal. */
/*In tests, you want two things happening at once:

The server listening for requests.

Your test logic continuing (sending requests, checking responses).

If you .await the server inside your test setup, the runtime will sit there forever — it’s busy serving requests and will never reach your assertions.

So instead, you use:

rust
tokio::spawn(server);
This tells the runtime:

“Run this future in the background while I do other things.” */
//ELABORATION OF WHY WE AWAIT FOR PRODUCTION AND WE WE SPAWN A THREAD IN THE TEST
//A future in Rust is not a thread.It’s a value that represents some async work. The runtime (Tokio) is responsible for polling futures and deciding when they make progress.
//When you .await a future, you’re saying: “Pause here until this future finishes.”
//HttpServer::run() returns a Server future.it doesn’t have a natural “done” state. It only stops if you kill the process.
/*
In tests, you need two things happening at once:

The server listening for requests.

Your test logic continuing (sending requests, checking responses).
That’s why you use: tokio::spawn(server);
This doesn’t create a new OS thread — it tells the Tokio runtime:

“Run this future in the background, alongside other futures.”
*/
/*Production (main.rs) → .await the server → block forever, keep serving users.

Tests (spawn_app) → tokio::spawn(server) → run concurrently, let test logic continue. */
