use std::borrow::Cow;

use crate::repl::parser::parse_command;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::Hinter;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Context, Editor, Helper};
use owo_colors::OwoColorize;

struct FangHelper;

impl Helper for FangHelper {}

impl Hinter for FangHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for FangHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        Cow::Borrowed(prompt)
    }

    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Borrowed(line)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Borrowed(hint)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _kind: CmdKind) -> bool {
        false
    }
}

impl Validator for FangHelper {
    fn validate(&self, _context: &mut ValidationContext<'_>) -> rustyline::Result<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }
}

impl Completer for FangHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        _line: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        Ok((0, Vec::new()))
    }
}

pub fn shell_main() {
    let mut editor = Editor::<FangHelper, rustyline::history::DefaultHistory>::new()
        .expect("Failed to initialize REPL editor");
    editor.set_helper(Some(FangHelper));

    println!("{}", "Welcome to the Fang REPL!".bold());
    println!("{}", "Type 'help' for a list of commands, or 'exit' to quit".dimmed());

    loop {
        match editor.readline("> ") {
            Ok(input) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                let _ = editor.add_history_entry(input);

                if input.eq_ignore_ascii_case("exit") {
                    println!("{}", "Goodbye!".green().bold());
                    break;
                } else if input.eq_ignore_ascii_case("help") {
                    crate::repl::commands::print_help();
                } else {
                    match parse_command(input) {
                        Ok(Some(command)) => crate::repl::commands::run_tool_by_path(&command.path, command.args).unwrap_or_else(|e| {
                            println!("Error: {e}");
                        }),
                        Ok(None) => continue,
                        Err(e) => println!("Error: {e}"),
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!();
                break;
            }
            Err(err) => {
                println!("Error: {err}");
                break;
            }
        }
    }
}