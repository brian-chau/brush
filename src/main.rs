mod builtins;
mod environment;
mod executor;
mod parser;
mod shell;
mod tokenizer;

fn main() {
    shell::run_shell();
}
