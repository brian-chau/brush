// Standard libraries
use std::process::Command as SysCommand;

// Project libraries
use crate::builtins;
use crate::parser::Command;

// External crates
// N/A

pub fn execute_command(
    cmd: Command,
    env: &mut crate::environment::Environment,
) -> Result<(), String> {
    // Handle built-ins
    if builtins::handle_builtin(&cmd.program, &cmd.args, env) {
        return Ok(());
    }

    // If it can't find it, use parent shell instead
    let status: std::process::ExitStatus = SysCommand::new(cmd.program)
        .args(cmd.args)
        .status()
        .map_err(|e| e.to_string())?;

    // If it still can't find it, then error out
    if !status.success() {
        Err(format!("Process exited with status: {}", status))
    } else {
        Ok(())
    }
}
