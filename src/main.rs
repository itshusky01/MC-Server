extern crate log;
extern crate log4rs;

use log::{LevelFilter, error, info, warn};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};
use chrono::Local;

use std::{env, io::Write};
use std::fs::File;
use server::{config::StartupConfig, server::Server};

const CONFIG: &str = 
"name = 'My Server!'
address = '127.0.0.1:2333'
max_players = 20
log_dir = './logs'
ext_dir = './exts'";

const LOG_FORMAT: &str = "[{d}][{l}] {m}{n}";

#[async_std::main]
async fn main() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_FORMAT)))
        .build();

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_FORMAT)))
        .build(format!("logs/{}.log", Local::now().timestamp_millis()))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(Logger::builder()
            .appender("file")
            .additive(false)
            .build("app", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    let _ = log4rs::init_config(config).unwrap();

    info!("Starting server");

    let mut file = "config.toml";

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        file = args.get(1).unwrap();
    } else {
        if !std::path::Path::new(file).exists() {
            warn!("The config file {} not exists, creating", file);
            let mut f = File::create(file).unwrap();
            f.write_all(CONFIG.as_bytes()).unwrap();
            warn!("created config file");
        }
    }

    let config = StartupConfig::from_file(file);
    let serv = Server::new(config).await;

    match serv {
        Err(err) => error!("{}", err),
        Ok(serv) => serv.run().await
    };

}
