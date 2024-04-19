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
use viper::Viper;
use cli::*;
use vir_to_viper::{encode_viper, ViperEncodeCtx};

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

    let mut v = Viper::new("");
    let vctx = v.attach_current_thread();
    let mut ast = vctx.new_ast_factory();
    
    let mut vctx = ViperEncodeCtx::new(ctx, ast);
    encode_viper(&mut vctx);

    Ok(())
}
