pub fn exit_with_error(error: eyre::Report, code: i32) -> ! {
  use crate::output::output_error_with_context;

  let message = error.to_string();
  let mut context = Vec::new();

  let mut source = error.source();
  while let Some(err) = source {
    context.push(format!("Caused by: {}", err));
    source = err.source();
  }

  if atty::is(atty::Stream::Stderr) {
    output_error_with_context(&message, &context);
  } else {
    eprintln!("Error: {}", message);
    for ctx in context {
      eprintln!("{}", ctx);
    }
  }

  std::process::exit(code);
}
