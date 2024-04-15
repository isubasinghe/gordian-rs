use std::sync::Arc;


type Ident = Arc<String>;

pub struct ParamX {
    name: Ident,
    ty: Typ
}

pub type Param = Arc<ParamX>;

pub struct FunctionDefX {
    params: Arc<Vec<Param>>,
    rettype: Typ
    
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
pub enum BinOpX {

}

pub type BinOp = Arc<BinOpX>;

#[derive(Debug)]
pub enum UOpX {
}

pub type UOp = Arc<UOpX>;

#[derive(Debug)]
pub enum StatementX {
    BinOp(BinOp),
    UOp(UOp),
    Call(Ident),
    Block(Statement)
}

pub type Statement = Arc<StatementX>;
