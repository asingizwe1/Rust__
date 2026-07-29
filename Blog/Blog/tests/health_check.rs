use zero2prod::main;

#[test]
fn dummy_test{

main()
}

//we restrefactor our project into library and a binary: : all our logic will live in the library crate while the binary itself will be just an entrypoint with a very slim main function.
// library crate (for reusable logic) and a binary crate (for the executable entrypoint).
/*Library crate (lib.rs) → contains reusable logic, functions, structs, modules.
 moving all your business logic, algorithms, and reusable components into the library crate, leaving the binary crate as a thin wrapper that just calls into the library.
Binary crate (main.rs) → contains the entrypoint (fn main()), typically minimal. */