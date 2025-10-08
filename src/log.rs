pub fn setup() {
    let colors = fern::colors::ColoredLevelConfig::new().info(fern::colors::Color::BrightCyan);
    let _ = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}{}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                record.module_path().map(|x| { format!("[{}] ", x) }).unwrap_or(String::new()),
                message
            ));
        })
        .level(log::LevelFilter::Error)
        .level_for(module_path!().split("::").next().unwrap(), log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply();
}
