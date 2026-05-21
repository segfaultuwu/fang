use owo_colors::OwoColorize;

pub fn color_namespace_path(path: &str) -> String {
    let segments = path.split("::").collect::<Vec<_>>();
    if segments.len() <= 1 {
        return path.cyan().bold().to_string();
    }

    let mut colored = String::new();
    for (index, segment) in segments.iter().enumerate() {
        if index > 0 {
            colored.push_str("::");
        }
        let colored_segment = match index % 6 {
            0 => segment.cyan().bold().to_string(),
            1 => segment.green().bold().to_string(),
            2 => segment.magenta().bold().to_string(),
            3 => segment.yellow().bold().to_string(),
            4 => segment.blue().bold().to_string(),
            _ => segment.bright_red().bold().to_string(),
        };
        colored.push_str(&colored_segment);
    }
    colored
}
