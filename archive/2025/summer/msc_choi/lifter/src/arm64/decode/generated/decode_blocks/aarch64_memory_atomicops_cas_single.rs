#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_memory_atomicops_cas_single(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let size = common::types::bits::from_bits_literal(
        &reader.extract_slice(30usize, 2usize)?,
    )?;
    let L = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 1usize)?,
    )?;
    let Rs = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 5usize)?,
    )?;
    let o0 = common::types::bits::from_bits_literal(
        &reader.extract_slice(15usize, 1usize)?,
    )?;
    let Rn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Rt = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Rn,
        common::types::integer::from(5),
    )?;
    let mut t: common::types::integer = decode::helpers::UInt_1(
        Rt,
        common::types::integer::from(5),
    )?;
    let mut s: common::types::integer = decode::helpers::UInt_1(
        Rs,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::aarch64_memory_atomicops_cas_single(
            Box::new(common::types::aarch64_memory_atomicops_cas_single_operands {
                n,
                s,
                t,
            }),
        ),
    )
}
