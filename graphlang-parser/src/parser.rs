use winnow::ascii::{space1, newline, space0};
use winnow::combinator::{alt, repeat, terminated};
use winnow::error::ErrMode;
use winnow::error::ErrorKind;
use winnow::error::ParserError;
use winnow::stream::AsChar;
use winnow::token::{one_of, take_while};
use winnow::PResult;
use winnow::Parser;

use crate::ast::Identifier;
use crate::ast::StructField;
use crate::ast::*;
use std::sync::Arc;

pub fn identifier<'s>(input: &mut &'s str) -> PResult<Identifier> {
    (
        one_of(|c: char| c.is_alpha() || c == '_'),
        take_while(0.., |c: char| c.is_alphanum() || c == '_'),
    )
        .recognize()
        .map(|v: &'s str| Arc::new(v.to_owned()))
        .parse_next(input)
}

pub fn parse_word_ty<'s>(input: &mut &'s str) -> PResult<PrimTyp> {
    let _ = "Word".parse_next(input)?;
    let _ = space1.parse_next(input)?;
    let pty = primtyp_raw.parse_next(input)?;
    Ok(pty)
}

pub fn parse_struct_field<'s>(input: &mut &'s str) -> PResult<StructField> {
    let _ = "StructField".parse_next(input)?;
    let _ = space1.parse_next(input)?;
    let id = identifier.parse_next(input)?;
    let _ = space1.parse_next(input)?;
    let ty = parse_ty.parse_next(input)?;
    let _ = space1.parse_next(input)?;
    let offset = parse_u64.parse_next(input)?;
    Ok(Arc::new(StructFieldX{name: id, ty, offset}))
}

pub fn parse_struct_def<'s>(input: &mut &'s str) -> PResult<Struct> {
    let _ = "Struct".parse_next(input)?;
    let _ = space1.parse_next(input)?;
    let sz = parse_u64.parse_next(input)?;
    let _ = space1.parse_next(input)?;
    let alignment = parse_u64.parse_next(input)?;
    let field = (newline, space0, parse_struct_field).map(|(_, _, f)| f);
    let fields: Vec<StructField> = repeat(0.., field).parse_next(input)?;
    Ok(Arc::new(StructX{sz, alignment, fields}))
}

pub fn parse_unit_ty<'s>(input: &mut &'s str) -> PResult<Typ> {
    "()".map(|_| Arc::new(TypX::UNIT)).parse_next(input)
}

pub fn parse_htd_ty<'s>(input: &mut &'s str) -> PResult<Typ> {
    "HTD".map(|_| Arc::new(TypX::HTD)).parse_next(input)
}

pub fn parse_mem_ty<'s>(input: &mut &'s str) -> PResult<Typ> {
    "Mem".map(|_| Arc::new(TypX::Mem)).parse_next(input)
}
pub fn parse_pms_ty<'s>(input: &mut &'s str) -> PResult<Typ> {
    "PMS".map(|_| Arc::new(TypX::PMS)).parse_next(input)
}

pub fn parse_bool_ty<'s>(input: &mut &'s str) -> PResult<Typ> {
    "Bool".map(|_| Arc::new(TypX::Bool)).parse_next(input)
}

pub fn parse_array_ty<'s>(input: &mut &'s str) -> PResult<Typ> {
    let _ = "Array".parse_next(input)?;
    let _ = space1.parse_next(input)?;
    let elem_ty = parse_ty.parse_next(input)?;
    let _ = space1.parse_next(input)?;
    let value_ty = parse_u64.parse_next(input)?;
    Ok(Arc::new(TypX::Array(elem_ty, value_ty)))
}

pub fn parse_word_array_ty<'s>(input: &mut &'s str) -> PResult<Typ> {
    let _ = "WordArray".parse_next(input)?;
    let _ = space1.parse_next(input)?;
    let index_bits = parse_u64.parse_next(input)?;
    let _ = space1.parse_next(input)?;
    let value_bits = parse_u64.parse_next(input)?;
    Ok(Arc::new(TypX::WordArray(index_bits, value_bits)))
}

pub fn parse_struct_ref_ty<'s>(input: &mut &'s str) -> PResult<Typ> {
    let _ = "Struct".parse_next(input)?;

    let _ = space1.parse_next(input)?;
    let id = identifier.parse_next(input)?;
    Ok(Arc::new(TypX::StructRef(id)))
}

pub fn parse_ty<'s>(input: &mut &'s str) -> PResult<Typ> {
    alt((
        parse_struct_ref_ty,
        parse_unit_ty,
        parse_htd_ty,
        parse_mem_ty,
        parse_pms_ty,
        parse_bool_ty,
        parse_array_ty,
        parse_word_array_ty,
    ))
    .parse_next(input)
}

pub fn parse_param<'s>(input: &mut &'s str) -> PResult<Param> {
    let id = identifier.parse_next(input)?;
    let _ = space1.parse_next(input)?;
    let ty = parse_ty.parse_next(input)?;
    Ok(Arc::new(ParamX{ty, varname: id}))
}

pub fn parse_function<'s>(input: &mut &'s str) -> PResult<Function> {
    todo!()
}

fn primtyp_raw<'s>(input: &mut &'s str) -> PResult<PrimTyp> {
    let d = decimal.parse_next(input)?;
    let p = match d {
        "64" => PrimTypX::Word64,
        "32" => PrimTypX::Word32,
        "16" => PrimTypX::Word16,
        "8" => PrimTypX::Word8,
        _ => return Err(ErrMode::from_error_kind(input, ErrorKind::Verify)),
    };
    return Ok(Arc::new(p));
}

fn parse_u64<'s>(input: &mut &'s str) -> PResult<u64> {
    decimal.parse_to().parse_next(input)
}

fn decimal<'s>(input: &mut &'s str) -> PResult<&'s str> {
    repeat(
        1..,
        terminated(one_of('0'..='9'), repeat(0.., '_').map(|()| ())),
    )
    .map(|()| ())
    .recognize()
    .parse_next(input)
}



pub fn parse_program<'s>(input: &mut &'s str) -> PResult<Program> {
    let struct_ = (newline, parse_struct_def).map(|(_, snd)| snd);
    let structs: Vec<Struct> = repeat(0.., struct_).parse_next(input)?;
    let function = (newline, parse_function).map(|(_, snd)| snd);
    let functions = repeat(0.., function).parse_next(input)?;
    Ok(Arc::new(ProgramX{structs, functions}))
}

