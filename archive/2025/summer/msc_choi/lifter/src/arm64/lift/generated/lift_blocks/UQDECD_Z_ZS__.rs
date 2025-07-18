#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{lift, common};
use crate::arm64::common::types::BigIntExt;
use crate::{integer_to_usize, integer_to_u32, integer_to_u64};
use crate::arm64::lift::types::{AirPackable, BlockSequencer};
use tnj::air::instructions::builder::InstructionBuilder;
use tnj::air::instructions::Value;
use tnj::arch::reg::Reg;
use tnj::types::Type;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn lift_UQDECD_Z_ZS__(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: lift::types::Variable,
    dn: common::types::integer,
) -> Result<(), AArch64LifterError> {
    let mut dn: lift::types::Variable = dn.into();
    let mut assigns_0: BTreeMap<String, lift::types::AirPackable> = BTreeMap::new();
    Ok(())
}
