use tokio::time::{sleep, Duration};

mod tools;
mod ui;
mod repl;
mod misc;

#[tokio::main]
async fn main() {
    ui::draw_main_menu();
}
