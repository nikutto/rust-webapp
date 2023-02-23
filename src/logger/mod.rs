pub fn init_logger() -> std::io::Result<()> {
    fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%+"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(std::io::stdout())
        .chain(fern::log_file("./logs/output.log")?)
        .apply()
        .unwrap();
    Ok(())
}
