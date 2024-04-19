use std::sync::Arc;

#[derive(Debug)]
pub struct Span<T> {
    pub pos: usize,
    pub t: T
}

type Ident = Arc<String>;

#[derive(Debug)]
pub struct ParamX {
    pub name: Ident,
    pub ty: Typ
}

pub type Param = Arc<ParamX>;

pub struct FunctionDefX {
    pub name: Ident,
    pub params: Arc<Vec<Param>>,
    pub stmt: Statement,
    pub rettype: Typ
    
}

pub type FunctionDef = Arc<FunctionDefX>;

#[derive(Copy, Clone, Debug)]
pub struct Int(pub u64);

#[derive(Copy, Clone, Debug)]
pub struct UInt(pub u64);

#[derive(Copy, Clone, Debug)]
pub enum TypX {
    UInt(UInt),
    Int(Int),
    Void,
    Bool
}

pub type Typ = Arc<TypX>;

#[derive(Debug)]
pub enum ExprX {
    BinOp(BinOp),
    UOp(UOp),
}

pub type Expr = Arc<ExprX>;

#[derive(Debug)]
pub enum BinOpX {
    Add(Expr, Expr),
    Sub(Expr, Expr),
}

pub type BinOp = Arc<BinOpX>;

#[derive(Debug)]
pub enum UOpX {
}

pub type UOp = Arc<UOpX>;

#[derive(Debug)]
pub enum StatementX {
    Call(Ident),
    Block(Statement),
    Assign(Ident, Expr),
    Return,
    ReturnE(Expr)
}

pub type Statement = Arc<Span<StatementX>>;
