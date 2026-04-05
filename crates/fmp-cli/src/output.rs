use eyre::Result;
use serde::Serialize;
use std::io::Write;

pub fn output_json<T: Serialize>(data: &T) -> Result<()> {
  let json = serde_json::to_string_pretty(data)?;
  println!("{}", json);
  Ok(())
}

pub fn output_error_with_context(message: &str, context: &[String]) {
  use owo_colors::OwoColorize;

  let stderr = std::io::stderr();
  let mut handle = stderr.lock();

  if atty::is(atty::Stream::Stderr) {
    writeln!(handle, "{} {}", "Error:".red().bold(), message).ok();
    for ctx in context {
      writeln!(handle, "  {}", ctx.bright_black()).ok();
    }
  } else {
    writeln!(handle, "Error: {}", message).ok();
    for ctx in context {
      writeln!(handle, "  {}", ctx).ok();
    }
  }
}
