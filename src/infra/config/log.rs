use chrono::Local;
use owo_colors::OwoColorize;

pub fn setup_logger() -> Result<(), fern::InitError> {
  fern::Dispatch::new()
    .level(log::LevelFilter::Debug)
    .chain(get_terminal_config())
    .chain(get_log_file_config()?)
    .apply()?;
  Ok(())
}

fn get_terminal_config() -> fern::Dispatch {
  fern::Dispatch::new()
    .format(|out, message, record| match record.level() {
      log::Level::Error => out.finish(format_args!(
        "{}: {}\n\t {}\n",
        record.level().bold().bright_red(),
        record.target().bold(),
        message,
      )),
      log::Level::Warn => out.finish(format_args!(
        "{}: {}\n\t {}\n",
        record.level().bold().yellow(),
        record.target().bold(),
        message,
      )),
      log::Level::Info => out.finish(format_args!(
        "{}: {}\n\t {}\n",
        record.level().bold().green(),
        record.target().bold(),
        message,
      )),
      log::Level::Debug => out.finish(format_args!(
        "{}: {}\n\t {}\n",
        record.level().bold().blue(),
        record.target().bold(),
        message,
      )),
      log::Level::Trace => out.finish(format_args!(
        "{}: {}\n\t {}\n",
        record.level().bold().white(),
        record.target().bold(),
        message,
      )),
    })
    .level(log::LevelFilter::Debug)
    .chain(std::io::stdout())
}

fn get_log_file_config() -> Result<fern::Dispatch, fern::InitError> {
  Ok(
    fern::Dispatch::new()
      .format(|out, message, record| {
        out.finish(format_args!(
          "[{}] {}: {} - {}",
          Local::now().format("%d-%m-%Y %H:%M:%S %z"),
          record.target(),
          record.level(),
          message,
        ))
      })
      .chain(fern::log_file(format!(
        "logs/{}.log",
        Local::now().date_naive()
      ))?),
  )
}
