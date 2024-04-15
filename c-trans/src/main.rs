use std::io;

use encoder::EncodeCtx;
use lang_c;
use lang_c::driver::{Config, parse};
mod cli;
mod encoder;
mod vir;
mod vir_to_viper;
use clap::Parser;
use cli::*;


fn main() -> io::Result<()> {
    let config = Config::default();
    let args = Args::parse();
    let mut ctx = EncodeCtx{};
    match parse(&config, args.file) {
        Ok(unit) => {
            ctx.encode(unit.unit);
        },
        Err(_) => {}
    }
    
    Ok(())
}
