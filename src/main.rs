// This file is kept for backward compatibility
// The actual server is now in src/bin/server.rs
// 
// To run the server: cargo run --bin server
// To create a superuser: cargo run --bin create_superuser

fn main() {
    eprintln!("This is the library crate. To run the server, use:");
    eprintln!("  cargo run --bin server");
    eprintln!("");
    eprintln!("To create a superuser, use:");
    eprintln!("  cargo run --bin create_superuser");
    std::process::exit(1);
}
