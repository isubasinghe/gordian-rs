use crate::{encoder::EncodeCtx, vir::FunctionDef};
use crate::vir::{self, BinOpX, UOpX};
use crate::vir::{TypX};
use std::sync::Arc;
use viper::AstFactory;
use viper::VerificationContext;
use viper::Viper;

pub enum LabelState {
    NoLabel,
    Block(u64)
}

pub struct ViperEncodeCtx<'a> {
    cctx: EncodeCtx,
    ast: AstFactory<'a>,
    label_state: LabelState,
}

impl<'a> ViperEncodeCtx<'a> {
    fn new_label(&mut self) -> String {
        let new_label = match self.label_state {
            LabelState::NoLabel => {
                self.label_state = LabelState::Block(1);
                "start".to_string()
            }
            LabelState::Block(n) => {
                let s = format!("blk{}", n);
                self.label_state = LabelState::Block(1 + n);
                s
            }
        };
        return new_label;
    }
}

pub trait ToViper<'v, T> {
    fn to_viper(&self, ctx: &mut ViperEncodeCtx<'v>, ast: &AstFactory<'v>) -> T;
}

// prelude



impl<'v> ToViper<'v, viper::Type<'v>> for vir::Typ {
    fn to_viper(&self, ctx: &mut ViperEncodeCtx<'v>, ast: &AstFactory<'v>) -> viper::Type<'v> {
        match **self {
            TypX::Void => todo!(),
            TypX::Bool => ast.bool_type(),
            TypX::Int(vir::Int(sz)) => ast.int_type(),
            TypX::UInt(vir::UInt(sz)) => ast.int_type(),
        }
    }
}

impl<'v> ToViper<'v, viper::LocalVarDecl<'v>> for vir::Param {
    fn to_viper(&self, ctx: &mut ViperEncodeCtx<'v>, ast: &AstFactory<'v>) -> viper::LocalVarDecl<'v> {
        ast.local_var_decl(&**self.name, self.ty.to_viper(ctx, &ast))
    }
}

impl<'v> ToViper<'v, viper::Expr<'v>> for vir::BinOp {
    fn to_viper(&self, ctx: &mut ViperEncodeCtx<'v>, ast: &AstFactory<'v>) -> viper::Expr<'v> {
        match &**self {
            BinOpX::Add(lhs, rhs) => ast.add(lhs.to_viper(ctx, &ast), rhs.to_viper(ctx, &ast)),
            BinOpX::Sub(lhs, rhs) => ast.sub(lhs.to_viper(ctx, &ast), rhs.to_viper(ctx, &ast))
        }
    }
}

impl<'v> ToViper<'v, viper::Expr<'v>> for vir::UOp {
    fn to_viper(&self, ctx: &mut ViperEncodeCtx<'v>, ast: &AstFactory<'v>) -> viper::Expr<'v> {
        unimplemented!("todo")
    }
}

impl<'v> ToViper<'v, viper::Expr<'v>> for vir::Expr {
    fn to_viper(&self, ctx: &mut ViperEncodeCtx<'v>, ast: &AstFactory<'v>) -> viper::Expr<'v> {
        match &**self {
            vir::ExprX::BinOp(binop) => binop.to_viper(ctx, ast),
            vir::ExprX::UOp(uop) => panic!("")
            
        }
    }
}


impl<'v> ToViper<'v, viper::Stmt<'v>> for vir::Statement {
    fn to_viper(&self, ctx: &mut ViperEncodeCtx<'v>, ast: &AstFactory<'v>) -> viper::Stmt<'v> {
        match &self.t {
            vir::StatementX::Block(ref stmt) => {
                let c = ctx.new_label();
                ast.label(&c, &[])
            },
            vir::StatementX::Assign(ref ident, ref expr) => {
                // ast.abstract_assign()
                unimplemented!()
            },
            vir::StatementX::Call(ref c) => panic!(""),
            vir::StatementX::Return => panic!(""),
            vir::StatementX::ReturnE(ref e) => panic!("")
        }
    }
}

impl<'a> ViperEncodeCtx<'a> {
    pub fn new(ctx: EncodeCtx, ast: AstFactory<'a>) -> ViperEncodeCtx<'a> {
        ViperEncodeCtx { cctx: ctx, ast, label_state: LabelState::NoLabel}
    }
}

pub fn encode_fn(ctx: &mut ViperEncodeCtx, vfn: FunctionDef) {
    // emit local vars
}

pub fn encode_viper(ctx: &mut ViperEncodeCtx) {
    // let vfns = vec![];

    for (cfn, fnctx) in &*ctx.cctx.fns.clone() {
        encode_fn(ctx, cfn.clone());
    }
}
