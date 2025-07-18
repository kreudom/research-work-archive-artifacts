#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_FMUL_Z_ZZi_H(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let i3h = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 1usize)?,
    )?;
    let i3l = common::types::bits::from_bits_literal(
        &reader.extract_slice(19usize, 2usize)?,
    )?;
    let Zm = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 3usize)?,
    )?;
    let Zn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Zd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Zn,
        common::types::integer::from(5),
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Zd,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::FMUL_Z_ZZi_H(
            Box::new(common::types::FMUL_Z_ZZi_H_operands {
                d,
                n,
            }),
        ),
    )
}
