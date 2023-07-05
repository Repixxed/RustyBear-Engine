
#[cfg(not(target_arch = "wasm32"))]
use simplelog::{ConfigBuilder, TermLogger};
#[cfg(not(target_arch = "wasm32"))]
use simplelog::{Level, Color, LevelFilter, TerminalMode, ColorChoice};

pub fn init()
{
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")]
        {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Failed to init logger.");
        }
        else
        {
            let config = ConfigBuilder::new()
            .set_level_color(Level::Trace, Some(Color::White))
            .set_level_color(Level::Info, Some(Color::Green))
            .set_level_color(Level::Warn, Some(Color::Yellow))
            .set_level_color(Level::Error, Some(Color::Red))
            .build();
    
            let _ = TermLogger::init(LevelFilter::Info, config, TerminalMode::Mixed, ColorChoice::Auto);
        }
    }
}