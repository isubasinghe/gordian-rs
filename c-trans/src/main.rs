use std::io;

use encoder::{EncodeCtx, encode};
use lang_c;
use lang_c::driver::{Config, parse};
mod cli;
mod encoder;
mod vir;
mod vir_to_viper;
mod tychk;
use clap::Parser;
use cli::*;

fn main() -> io::Result<()> {
    let config = Config::default();
    let args = Args::parse();
    let mut ctx = EncodeCtx::new();

    match parse(&config, args.file) {
        Ok(unit) => {
            encode(&mut ctx, unit.unit);
        },
        Err(_) => panic!("")
    };


    
    Ok(())
}
