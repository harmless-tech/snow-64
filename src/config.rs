use anyhow::*;
use configparser::ini;

pub const DISPLAY_RES: u32 = 256;

#[cfg(debug_assertions)]
pub const DEBUG_BUILD: bool = true;
#[cfg(not(debug_assertions))]
pub const DEBUG_BUILD: bool = false;

// TODO: Move.
pub struct Snow64Config {
    pub display_res: u32,
    pub dev_debug: bool,
}
impl Default for Snow64Config {
    fn default() -> Self {
        Self {
            display_res: DISPLAY_RES,
            dev_debug: DEBUG_BUILD,
        }
    }
}

pub fn load_config() -> Result<Snow64Config> {
    let mut config = Snow64Config::default();

    let file = std::fs::read_to_string("./snow64-data/config.ini").unwrap_or("".to_string());
    if !file.is_empty() {
        let mut parser = ini::Ini::new();
        parser.read(file).map_err(|e| Error::msg(e))?;

        match parser
            .getuint("display", "res")
            .map_err(|e| Error::msg(e))?
        {
            None => {}
            Some(val) => {
                if val as u32 >= DISPLAY_RES {
                    config.display_res = val as u32;
                }
            }
        }
        match parser.getbool("dev", "debug").map_err(|e| Error::msg(e))? {
            None => {}
            Some(val) => config.dev_debug = val,
        }
    }

    // Other debug stuff
    let mut args = std::env::args();
    let debug = DEBUG_BUILD || args.any(|s| s.eq("--dbg")) || config.dev_debug;
    let debug = debug && !(args.any(|s| s.eq("--no-dbg")));

    Ok(config)
}
