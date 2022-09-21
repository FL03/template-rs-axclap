/*
    Appellation: core <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::core::*;

mod core;


#[tokio::main]
async fn main() -> scsys::BoxResult {
    println!("Hello, world!");

    let mut app = Interface::new();
    app.cli().expect("Failed to run the application...");

    Ok(())
}
