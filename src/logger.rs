#[cfg(debug_assertions)]
pub fn init_logger() {
    use log::LevelFilter;
    use log4rs::config::{Appender, Config, Root};

    let file_appender = Appender::builder().build(
        "file",
        Box::new(
            log4rs::append::file::FileAppender::builder()
                .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                    "{d} [{l}] {m}{n}",
                )))
                .build("app.log")
                .unwrap(),
        ),
    );

    let config = Config::builder()
        .appender(file_appender)
        .build(Root::builder().appender("file").build(LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
