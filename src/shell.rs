// Standard libraries
use std::io::{self, Write};
use std::process;

// Project libraries
use crate::environment::Environment;
use crate::executor::execute_command;
use crate::parser::{expand_variables, parse_command};

// External crates
use crossterm::{
    cursor::{self, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    ExecutableCommand,
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};

pub fn run_shell() {
    let mut stdout: io::Stdout = io::stdout();
    let mut env: Environment = Environment::new();
    let mut history: Vec<String> = Vec::new();
    let mut history_index: Option<usize> = None;

    enable_raw_mode().unwrap();

    loop {
        // Prompt
        stdout.execute(cursor::MoveToColumn(0)).unwrap();
        print!("> ");
        stdout.flush().unwrap();

        let mut buffer: String = String::new();

        // Handle raw input
        loop {
            if event::poll(std::time::Duration::from_millis(100)).expect("Unable to poll") {
                if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read().unwrap() {
                match (code, modifiers) {
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) |
                        (KeyCode::Char('z'), KeyModifiers::CONTROL) |
                        (KeyCode::Char('x'), KeyModifiers::CONTROL) => {
                            let mut stdout: io::Stdout = io::stdout();
                            let _ = disable_raw_mode();
                            let _ = execute!(stdout, LeaveAlternateScreen, Show);
                            let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
                            let _ = stdout.flush();

                            process::exit(0);
                        }
                        (KeyCode::Char(c), _) => {
                            buffer.push(c);
                            print!("{}", c);
                            stdout.flush().unwrap();
                        }
                        (KeyCode::Backspace, _) => {
                            if !buffer.is_empty() {
                                buffer.pop();
                                print!("{} {}", 8 as char, 8 as char);
                                stdout.flush().unwrap();
                            }
                        }
                        (KeyCode::Enter, _) => {
                            print!("\r\n");
                            stdout.flush().unwrap();
                            break;
                        }
                        (KeyCode::Up, _) => {
                            if let Some(new_index) =
                                history_index.map(|i| i.saturating_sub(1)).or_else(|| {
                                    if !history.is_empty() {
                                        Some(history.len() - 1)
                                    } else {
                                        None
                                    }
                                })
                            {
                                history_index = Some(new_index);
                                buffer = history[new_index].clone();
                                redraw_input_line(&buffer);
                            }
                        }
                        (KeyCode::Down, _) => {
                            if let Some(index) = history_index {
                                let next_index = index + 1;
                                if next_index < history.len() {
                                    history_index = Some(next_index);
                                    buffer = history[next_index].clone();
                                } else {
                                    history_index = None;
                                    buffer.clear();
                                }
                                redraw_input_line(&buffer);
                            }
                        }
                        (KeyCode::Esc, _) => {
                            println!("\nExiting shell.");
                            println!("\033[2J\033[H\033[?1049l");
                            disable_raw_mode().unwrap();
                            io::stdout().flush().expect("Error msg");
                            return;
                        }
                        _ => {}
                    }
                }
            }
        }

        history.push(buffer.clone());
        history_index = None;

        // Expand variables in general command line
        let expanded_input: String = expand_variables(&buffer, &env);

        // Parse and execute
        if let Some(command) = parse_command(&expanded_input) {
            if let Err(e) = execute_command(command, &mut env) {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn redraw_input_line(buffer: &str) {
    let mut stdout: io::Stdout = io::stdout();

    // Move cursor to beginning of line and clear line
    // \r = carriage return, \x1B[K = ANSI escape to clear line from cursor right
    write!(stdout, "\r\x1B[K> {}", buffer).expect("error msg");

    stdout.flush().expect("Err msg");
}
