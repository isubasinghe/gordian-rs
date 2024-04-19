use std::sync::Arc;
use std::cell::Cell;

use lang_c::{ast::*, span::Node};
use crate::vir::{self, *};


pub struct FnCtxt {

}

impl FnCtxt {
    pub fn new() -> FnCtxt {
        FnCtxt {  }
    }

    fn get_fn_name(decl: Node<FunctionDefinition>) {
        let name = decl.node.specifiers;
    }

    fn typ_from_var(name: &str) -> Typ {
        unimplemented!()
    }

    fn get_param(&self, param: Node<ParameterDeclaration>) -> Param {
        let typ = match &param.node.specifiers[0].node {
            DeclarationSpecifier::TypeSpecifier(nty) => from_typspec(nty.clone()),
            _ => panic!("")
        };

        let decl = match param.node.declarator {
            Some(idn) => idn.node.kind.node,
            None => panic!("")
        };

        let ident = match decl {
            DeclaratorKind::Identifier(ident) => ident.node.name,
            _ => panic!("")
        };

        let param = ParamX{name: ident.into(), ty: typ};
        Arc::new(param)

    }


    fn encode_stmt(&self, stmt: Node<lang_c::ast::Statement>) -> vir::Statement {
        unimplemented!();
    }

    pub fn init_fn(&mut self, decl: Node<FunctionDefinition>) -> FunctionDef {

        let typ = match &decl.node.specifiers[0].node {
            DeclarationSpecifier::TypeSpecifier(nty)  => from_typspec(nty.clone()),
            _ => panic!("")
        };


        let ident = match decl.clone().node.declarator.node.kind.node {
            DeclaratorKind::Identifier(ident) => ident.node,
            _ => panic!("")
        };


        let fnp = match &decl.node.declarator.node.derived[0].node {
            DerivedDeclarator::Function(nfn) => nfn.clone().node,
            _ => panic!("")
        };

        let mut params = vec![];

        for param in fnp.parameters {
            let converted_param = self.get_param(param);
            params.push(converted_param);
        }

        let vstmt = self.encode_stmt(decl.node.statement);

        let mut fndef = FunctionDefX{name: Arc::new(ident.name), params: Arc::new(params), rettype: typ, stmt: vstmt};
        Arc::new(fndef)
    }
}

pub struct EncodeCtx {
    pub val: u32,
    pub fns: Arc<Vec<(FunctionDef, FnCtxt)>>
}

impl EncodeCtx {
}

impl EncodeCtx {
    pub fn new() -> EncodeCtx {
        let fns = Arc::new(vec![]);
        EncodeCtx { val: 0, fns }
    }
}

pub fn from_typspec( nspec: Node<TypeSpecifier>) -> Typ {
    match nspec.node {
        TypeSpecifier::Int => Arc::new(TypX::Int(Int(32))),
        TypeSpecifier::Void => Arc::new(TypX::Void),
        TypeSpecifier::Char => Arc::new(TypX::UInt(UInt(8))),
        TypeSpecifier::Long => Arc::new(TypX::Int(Int(64))),
        TypeSpecifier::Bool => Arc::new(TypX::Bool),
        TypeSpecifier::Short => Arc::new(TypX::Int(Int(16))),
        TypeSpecifier::Signed => Arc::new(TypX::Int(Int(32))),
        TypeSpecifier::Unsigned => Arc::new(TypX::UInt(UInt(32))),
        _ => panic!("Not supported")
    }
}
pub fn encode(ctx: &mut EncodeCtx, unit: TranslationUnit) {
    let mut fn_to_encode = vec![];
    for extdef in unit.0 {
        match extdef.node {
            ExternalDeclaration::Declaration(ndecl) => {
            },
            ExternalDeclaration::StaticAssert(nstat) => {
                panic!("not supported");
            },
            ExternalDeclaration::FunctionDefinition(nfunc) => {
                fn_to_encode.push(nfunc)
            }
        }
    }

    for fndef in fn_to_encode {
        encode_fn(ctx, fndef);
    }
}

pub fn get_typ(ctx: &mut EncodeCtx, specifiers: Vec<Node<DeclarationSpecifier>>) -> Typ {
    match specifiers.len() {
        0 => panic!("expected at least one"),
        1 => {
            match &specifiers[0].node {
                DeclarationSpecifier::TypeSpecifier(ntyspec) => from_typspec(ntyspec.clone()),
                _ => panic!("")
            }
        },
        _ => panic!("not yet supported")
    }
}

pub fn encode_fn(ctx: &mut EncodeCtx, fndef: Node<FunctionDefinition>) ->  (FunctionDef, FnCtxt){

    let mut fnctx = FnCtxt::new();

    if fndef.node.declarations.len() != 0 {
        panic!("K&R style declarations are not supported");
    }

    fnctx.init_fn(fndef.clone());

    unimplemented!();
}

pub fn encode_struct(ctx: &mut EncodeCtx) {
}

