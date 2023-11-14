use std::sync::Arc;

pub enum UOpX {

}
pub type UOp = Arc<UOpX>;

pub enum BinOpX {
    Eq(Expr, Expr),
    Le(Expr, Expr),
    Lt(Expr, Expr),
    Ge(Expr, Expr),
    Gt(Expr, Expr),
    BWAnd(Expr, Expr),
    BWOr(Expr, Expr),
    BWLSH(Expr, Expr),
    BWRSH(Expr, Expr)
}

pub type BinOp = Arc<BinOpX>;

pub enum TriOpX {
    PValid(Expr, Typ, Expr),
}

pub type TriOp = Arc<TriOpX>;

pub enum ExprX {
    ExprSymbol(Identifier),
    ExprNum(PrimTyp, Num),
}
pub type Expr = Arc<ExprX>;

pub enum Num {
    Signed(i64),
    Unsigned(u64),
}

pub struct Spanned {}

pub struct ParamX {
    pub ty: Typ,
    pub varname: Identifier
}

pub type Param = Arc<ParamX>;

pub struct FunctionX {}

pub type Function = Arc<FunctionX>;

pub type Identifier = Arc<String>;

pub struct StructFieldX {
    pub name: Identifier,
    pub ty: Typ,
    pub offset: u64
}

pub type StructField = Arc<StructFieldX>;

pub enum NodeX {
    Basic(),
}

pub enum PrimTypX {
    Word64,
    Word32,
    Word16,
    Word8,
}

pub type PrimTyp = Arc<PrimTypX>;

pub enum TypX {
    PrimTyp(PrimTyp),
    StructRef(Identifier),
    Array(Typ, u64),
    WordArray(u64, u64),
    Bool,
    Mem,
    HTD,
    PMS,
    UNIT,
}

pub type Typ = Arc<TypX>;

pub type Node = Arc<NodeX>;

pub struct StructX {
    pub sz: u64,
    pub alignment: u64,
    pub fields: Vec<StructField>
}

pub type Struct = Arc<StructX>;



pub struct ProgramX {
    pub structs: Vec<Struct>,
    pub functions: Vec<Function>
}


pub type Program =  Arc<ProgramX>;



