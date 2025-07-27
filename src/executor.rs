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
    if builtins::handle_builtin(&cmd.program, &cmd.args, env) {
        return Ok(());
    }

    let status: std::process::ExitStatus = SysCommand::new(cmd.program)
        .args(cmd.args)
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        Err(format!("Process exited with status: {}", status))
    } else {
        Ok(())
    }
}
