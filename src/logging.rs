use log::{info, warn, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};
use std::fs::remove_file;

//TODO Allow for log path to be changed?
static LOG_PATH: &str = "snow-64-data/snow-64.log";

pub fn setup_log(debug: bool) -> log4rs::Handle {
    // Cleanup
    let cleanup_log= match remove_file(LOG_PATH) {
        Ok(_) => true,
        Err(_) => false,
    };

    let filter = if debug { LevelFilter::Trace } else { LevelFilter::Info };

    // Setup
    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%H:%M:%S)(local)} - {l}: {m}{n})}",
        )))
        .build();

    let fileout: FileAppender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%H:%M:%S)(local)} - {l}: {m}{n})}",
        )))
        .build(LOG_PATH)
        .unwrap();

    let config: Config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("fileout", Box::new(fileout)))
        .logger(Logger::builder().build("app::backend::db", LevelFilter::Trace))
        .logger(
            Logger::builder()
                .appender("fileout")
                .additive(false)
                .build("app::fileout", filter),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("fileout")
                .build(filter),
        )
        .unwrap();

    let handle: log4rs::Handle = log4rs::init_config(config).unwrap();

    if cleanup_log {
        info!("Previous log file deleted.")
    }
    else {
        warn!("Previous log file could not be deleted. This could become a problem if the log file gets very long.")
    }

    return handle;
}
