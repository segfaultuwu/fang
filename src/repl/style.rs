use owo_colors::OwoColorize;

pub fn color_prompt(prompt: &str) -> String {
    format!("{}{}", "fang".bright_cyan().bold(), prompt.dimmed())
}

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

pub fn color_input_line(line: &str) -> String {
    let mut output = String::new();
    let mut chars = line.chars().peekable();
    let mut first_token = true;

    while let Some(ch) = chars.peek().copied() {
        if ch.is_whitespace() {
            output.push(ch);
            chars.next();
            continue;
        }

        let mut token = String::new();
        let mut quote = None;

        while let Some(current) = chars.peek().copied() {
            if quote.is_none() && current.is_whitespace() {
                break;
            }

            token.push(current);
            chars.next();

            if current == '\\' {
                if let Some(escaped) = chars.next() {
                    token.push(escaped);
                }
                continue;
            }

            if current == '"' || current == '\'' {
                quote = match quote {
                    Some(active) if active == current => None,
                    Some(active) => Some(active),
                    None => Some(current),
                };
            }
        }

        let styled = if first_token {
            color_namespace_path(&token)
        } else if token.starts_with("--") {
            token.magenta().bold().to_string()
        } else if token.starts_with('"') || token.starts_with('\'') {
            token.yellow().to_string()
        } else if token.parse::<f64>().is_ok() {
            token.cyan().bold().to_string()
        } else if token.contains('=') {
            let mut parts = token.splitn(2, '=');
            let key = parts.next().unwrap_or_default();
            let value = parts.next().unwrap_or_default();
            format!("{}{}{}", key.green().bold(), "=".dimmed(), value.yellow())
        } else {
            token.white().to_string()
        };

        first_token = false;
        output.push_str(&styled);
    }

    output
}