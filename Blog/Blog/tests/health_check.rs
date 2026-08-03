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
async fn spawn_app() -> std::io::Result<()> {
    zero2prod::run().await //package name in unit test
}
async fn health_check_works() {
    // Arrange  -> spawn_app piece is the only piece that will depend on our application code
    spawn_app().await.expect("Failed to spawn our app.");
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
// Launch our application in the background ~somehow~
async fn spawn_app() -> std::io::Result<()> {
    todo!()
}
//we restrefactor our project into library and a binary: : all our logic will live in the library crate while the binary itself will be just an entrypoint with a very slim main function.
// library crate (for reusable logic) and a binary crate (for the executable entrypoint).
/*Library crate (lib.rs) → contains reusable logic, functions, structs, modules.

 moving all your business logic, algorithms, and reusable components into the library crate, leaving the binary crate as a thin wrapper that just calls into the library.

Binary crate (main.rs) → contains the entrypoint (fn main()), typically minimal. */
