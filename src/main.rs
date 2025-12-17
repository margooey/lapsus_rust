pub mod helper;
pub mod engine;

fn main() {
    helper::fix_cursor();
    engine::Engine::new();
    println!("Hello, world!");
}
