use std::sync::Arc;
use crate::{encoder::EncodeCtx, vir::FunctionDef};
use viper::AstFactory;

pub struct ViperEncodeCtx {
    cctx: EncodeCtx,
}


impl ViperEncodeCtx {
    pub fn new(ctx: EncodeCtx) -> ViperEncodeCtx {
        ViperEncodeCtx { cctx: ctx }
    }
}


pub fn encode_fn(ctx: &mut ViperEncodeCtx, vfn: FunctionDef)  {

}

pub fn encode_viper(ctx: &mut ViperEncodeCtx) {
    // let vfns = vec![];

    for cfn in &*ctx.cctx.fns.get_mut() {
        
    }
}
