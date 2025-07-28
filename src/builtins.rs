// Standard libraries
use std::env;
use std::io::{self, Write};
use std::process;

// Project libraries
use crate::environment::Environment;
use crate::parser::expand_variables;

// External crates
use crossterm::{
    cursor::{MoveTo, Show},
    execute,
    terminal::{Clear, ClearType, LeaveAlternateScreen, disable_raw_mode},
};

pub fn handle_builtin(program: &str, args: &[String], env: &mut Environment) -> bool {
    match program {
        "cd" => {
            let dir: &str = args.get(0).map(|s| s.as_str()).unwrap_or("/");
            if let Err(e) = env::set_current_dir(dir) {
                eprintln!("cd: {}", e);
            }
            true
        }

        "exit" | "quit" => {
            let mut stdout: io::Stdout = io::stdout();
            let _ = disable_raw_mode();
            let _ = execute!(stdout, LeaveAlternateScreen, Show);
            let _ = execute!(stdout, Clear(ClearType::All), MoveTo(0, 0));
            let _ = stdout.flush();

            process::exit(0);
        }

        "pwd" => {
            match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("pwd: {}", e),
            }
            true
        }

        "clear" => {
            // ANSI escape code to clear screen
            print!("\x1B[2J\x1B[1;1H");
            io::stdout().flush().unwrap();
            true
        }

        "set" => {
            for (key, val) in env.iter() {
                println!("{}={}", key, val);
            }
            true
        }

        "export" => {
            // Handle export: export VAR=value
            let parts: Vec<(String, String)> = args
                .iter()
                .map(|s| {
                    let mut parts = s.splitn(2, '=');
                    let key: String = parts.next().unwrap().to_string();
                    let value: String =
                        parts.next().map(|v| v.to_string()).expect("Invalid export");
                    (key, value)
                })
                .collect();

            if parts.len() == 1 {
                let key: &str = parts[0].0.as_str();
                let val: &str = parts[0].1.as_str();

                env.set_var(key, val);
            }
            true
        }
        "echo" => {
            let expanded: String = expand_variables(&args.join(" "), &env);
            println!("{}", expanded);

            true
        }
        _ => false,
    }
}
