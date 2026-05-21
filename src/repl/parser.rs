#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub path: Vec<String>,
    pub args: Vec<String>,
}

pub fn lex(input: &str) -> anyhow::Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let chars = input.chars().peekable();
    let mut in_quotes = false;
    let mut quote_char = '\0';
    let mut escaped = false;

    for ch in chars {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }

        match ch {
            '\\' => escaped = true,
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = ch;
            }
            ch if in_quotes && ch == quote_char => {
                in_quotes = false;
            }
            ch if ch.is_whitespace() && !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            ch => current.push(ch),
        }
    }

    if escaped {
        return Err(anyhow::anyhow!("Trailing escape character"));
    }

    if in_quotes {
        return Err(anyhow::anyhow!("Unclosed quoted string"));
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}

pub fn parse_command(input: &str) -> anyhow::Result<Option<Command>> {
    let tokens = lex(input)?;
    if tokens.is_empty() {
        return Ok(None);
    }

    let path = tokens[0]
        .split("::")
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.to_string())
        .collect::<Vec<_>>();

    if path.is_empty() {
        return Err(anyhow::anyhow!("Invalid command path"));
    }

    Ok(Some(Command {
        path,
        args: tokens[1..].to_vec(),
    }))
}
