use owo_colors::OwoColorize;
use crate::repl;

mod terms;

pub fn draw_ascii() {
    println!("{}{}{}", r#"
  █████▒▄▄▄       ███▄    █   ▄████ 
▓██   ▒▒████▄     ██ ▀█   █  ██▒ ▀█▒
▒████ ░▒██  ▀█▄  ▓██  ▀█ ██▒▒██░▄▄▄░
░▓█▒  ░░██▄▄▄▄██ ▓██▒  ▐▌██▒░▓█  ██▓
░▒█░    ▓█   ▓██▒▒██░   ▓██░░▒▓███▀▒
 ▒ ░    ▒▒   ▓▒█░░ ▒░   ▒ ▒  ░▒   ▒ 
 ░  ░     ▒   ▒▒ ░░ ░░   ░ ▒░  ░   ░ 
 ░ ░   By segfaultuwu   ░ ░ ░ ░   ░ 
 ░  ░     Version: "#.red().bold(), env!("CARGO_PKG_VERSION").purple(), " ░  ░ ░".red().bold());
}

pub fn draw_main_menu() {
    draw_ascii();
    println!();
    println!("{}", "Disclaimer: You must agree to the terms and conditions before using this software.".yellow().bold());
    terms::draw_terms();
    let input = rprompt::prompt_reply("Do you agree to the terms and conditions? (yes/no): ").expect("Failed to read input");
    if input.trim().eq_ignore_ascii_case("yes") {
        println!("{}", "Thank you for agreeing to the terms and conditions. You may now use the software.".green().bold());
        println!();
        repl::shell::shell_main();
    } else {
        println!("{}", "You must agree to the terms and conditions to use this software. Exiting...".red().bold());
    }
    println!();
}
