#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_ORR_Z_ZI__(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let imm13 = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 13usize)?,
    )?;
    let Zdn = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut dn: common::types::integer = decode::helpers::UInt_1(
        Zdn,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::ORR_Z_ZI__(
            Box::new(common::types::ORR_Z_ZI___operands {
                dn,
            }),
        ),
    )
}
