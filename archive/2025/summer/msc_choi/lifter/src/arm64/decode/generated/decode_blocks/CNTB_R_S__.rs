#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_CNTB_R_S__(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let imm4 = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 4usize)?,
    )?;
    let pattern = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Rd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Rd,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::CNTB_R_S__(
            Box::new(common::types::CNTB_R_S___operands {
                d,
            }),
        ),
    )
}
