use owo_colors::OwoColorize;

pub fn draw_terms() {
    println!("{}", "Terms and Conditions".yellow().bold());
    println!("{}. By using this software, you agree to the following terms and conditions:", "1".bright_purple().bold());
    println!("{}. You will not use this software for any illegal or malicious activities.", "2".bright_purple().bold());
    println!("{}. The author is not responsible for any damage or loss caused by the use of this software.", "3".bright_purple().bold());
    println!("{}. You are responsible for ensuring that you have the necessary permissions to use this software on any target systems.", "4".bright_purple().bold());
    println!("{}. You will not distribute this software without the author's permission.", "5".bright_purple().bold());
    println!("{}. The author reserves the right to modify these terms and conditions at any time.", "6".bright_purple().bold());
}