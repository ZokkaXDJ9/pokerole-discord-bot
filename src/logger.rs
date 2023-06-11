use env_logger::Builder;
use log::{LevelFilter};

pub fn init_logging() {
    Builder::new()
        .format_module_path(true)
        .filter(None, LevelFilter::Warn)
        .filter_module("pokerole_discord_bot", LevelFilter::max()).init();
}
