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
pub fn lift_aarch64_integer_arithmetic_mul_uniform_add_sub(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: lift::types::Variable,
    a: common::types::integer,
    d: common::types::integer,
    datasize: common::types::integer,
    destsize: common::types::integer,
    m: common::types::integer,
    n: common::types::integer,
    sub_op: common::types::boolean,
) -> Result<(), AArch64LifterError> {
    let mut a: lift::types::Variable = a.into();
    let mut d: lift::types::Variable = d.into();
    let mut datasize: lift::types::Variable = datasize.into();
    let mut destsize: lift::types::Variable = destsize.into();
    let mut m: lift::types::Variable = m.into();
    let mut n: lift::types::Variable = n.into();
    let mut sub_op: lift::types::Variable = sub_op.into();
    let mut assigns_0: BTreeMap<String, lift::types::AirPackable> = BTreeMap::new();
    let mut operand1: lift::types::Variable = {
        let arg_0 = n.clone();
        let arg_1 = datasize.clone();
        lift::helpers::X_read_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
    };
    let mut operand2: lift::types::Variable = {
        let arg_0 = m.clone();
        let arg_1 = datasize.clone();
        lift::helpers::X_read_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
    };
    let mut operand3: lift::types::Variable = {
        let arg_0 = a.clone();
        let arg_1 = destsize.clone();
        lift::helpers::X_read_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
    };
    let mut result: lift::types::Variable = lift::types::Variable::from(
        common::types::integer::default(),
    );
    {
        let cond = sub_op.clone();
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    result = {
                        let arg_0 = {
                            let arg_0 = operand3.clone();
                            let arg_1 = destsize.clone();
                            lift::helpers::UInt_1(
                                builder,
                                sequencer,
                                pc.clone(),
                                arg_0,
                                arg_1,
                            )?
                        };
                        let arg_1 = ({
                            let arg_0 = {
                                let arg_0 = operand1.clone();
                                let arg_1 = datasize.clone();
                                lift::helpers::UInt_1(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            };
                            let arg_1 = {
                                let arg_0 = operand2.clone();
                                let arg_1 = datasize.clone();
                                lift::helpers::UInt_1(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            };
                            lift::helpers::mul_int_0(
                                builder,
                                sequencer,
                                pc.clone(),
                                arg_0,
                                arg_1,
                            )?
                        });
                        lift::helpers::sub_int_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                        )?
                    };
                } else {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    result = {
                        let arg_0 = {
                            let arg_0 = operand3.clone();
                            let arg_1 = destsize.clone();
                            lift::helpers::UInt_1(
                                builder,
                                sequencer,
                                pc.clone(),
                                arg_0,
                                arg_1,
                            )?
                        };
                        let arg_1 = ({
                            let arg_0 = {
                                let arg_0 = operand1.clone();
                                let arg_1 = datasize.clone();
                                lift::helpers::UInt_1(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            };
                            let arg_1 = {
                                let arg_0 = operand2.clone();
                                let arg_1 = datasize.clone();
                                lift::helpers::UInt_1(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            };
                            lift::helpers::mul_int_0(
                                builder,
                                sequencer,
                                pc.clone(),
                                arg_0,
                                arg_1,
                            )?
                        });
                        lift::helpers::add_int_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                        )?
                    };
                }
            }
            lift::types::Variable::Air(a_inner) => {
                let current_block = builder.current_block();
                let then_block = sequencer
                    .get_block(
                        pc.to_bits()?.value as u64,
                        lift::types::BlockType::IntraBlock,
                        builder,
                        &vec![],
                    )?;
                builder.set_insert_block(then_block);
                let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                    .clone();
                assigns_1
                    .insert(
                        "result".to_string(),
                        {
                            let arg_0 = {
                                let arg_0 = match assigns_1.get("operand3") {
                                    Some(packable) => (*packable).clone().try_into()?,
                                    None => operand3.clone(),
                                };
                                let arg_1 = match assigns_1.get("destsize") {
                                    Some(packable) => (*packable).clone().try_into()?,
                                    None => destsize.clone(),
                                };
                                lift::helpers::UInt_1(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            };
                            let arg_1 = ({
                                let arg_0 = {
                                    let arg_0 = match assigns_1.get("operand1") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => operand1.clone(),
                                    };
                                    let arg_1 = match assigns_1.get("datasize") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => datasize.clone(),
                                    };
                                    lift::helpers::UInt_1(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                        arg_1,
                                    )?
                                };
                                let arg_1 = {
                                    let arg_0 = match assigns_1.get("operand2") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => operand2.clone(),
                                    };
                                    let arg_1 = match assigns_1.get("datasize") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => datasize.clone(),
                                    };
                                    lift::helpers::UInt_1(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                        arg_1,
                                    )?
                                };
                                lift::helpers::mul_int_0(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            });
                            lift::helpers::sub_int_0(
                                builder,
                                sequencer,
                                pc.clone(),
                                arg_0,
                                arg_1,
                            )?
                        }
                            .into(),
                    );
                let (then_args, block_param_types): (Vec<Value>, Vec<Type>) = vec![
                    assigns_1.get("result").unwrap()
                    .unpack_to_air_values_and_types(builder) ?
                ]
                    .into_iter()
                    .flat_map(|(args, tys): (Vec<Value>, Vec<Type>)| {
                        args.into_iter().zip(tys.into_iter())
                    })
                    .unzip();
                let else_block = sequencer
                    .get_block(
                        pc.to_bits()?.value as u64,
                        lift::types::BlockType::IntraBlock,
                        builder,
                        &vec![],
                    )?;
                builder.set_insert_block(current_block);
                builder.jumpif(a_inner.val, then_block, [], else_block, []);
                builder.set_insert_block(else_block);
                let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                    .clone();
                assigns_1
                    .insert(
                        "result".to_string(),
                        {
                            let arg_0 = {
                                let arg_0 = match assigns_1.get("operand3") {
                                    Some(packable) => (*packable).clone().try_into()?,
                                    None => operand3.clone(),
                                };
                                let arg_1 = match assigns_1.get("destsize") {
                                    Some(packable) => (*packable).clone().try_into()?,
                                    None => destsize.clone(),
                                };
                                lift::helpers::UInt_1(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            };
                            let arg_1 = ({
                                let arg_0 = {
                                    let arg_0 = match assigns_1.get("operand1") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => operand1.clone(),
                                    };
                                    let arg_1 = match assigns_1.get("datasize") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => datasize.clone(),
                                    };
                                    lift::helpers::UInt_1(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                        arg_1,
                                    )?
                                };
                                let arg_1 = {
                                    let arg_0 = match assigns_1.get("operand2") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => operand2.clone(),
                                    };
                                    let arg_1 = match assigns_1.get("datasize") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => datasize.clone(),
                                    };
                                    lift::helpers::UInt_1(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                        arg_1,
                                    )?
                                };
                                lift::helpers::mul_int_0(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            });
                            lift::helpers::add_int_0(
                                builder,
                                sequencer,
                                pc.clone(),
                                arg_0,
                                arg_1,
                            )?
                        }
                            .into(),
                    );
                let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                    assigns_1.get("result").unwrap()
                    .unpack_to_air_values_and_types(builder) ?
                ]
                    .into_iter()
                    .flat_map(|(args, tys): (Vec<Value>, Vec<Type>)| {
                        args.into_iter().zip(tys.into_iter())
                    })
                    .unzip();
                let end_block = sequencer
                    .get_block(
                        pc.to_bits()?.value as u64,
                        lift::types::BlockType::IntraBlock,
                        builder,
                        &block_param_types,
                    )?;
                builder.set_insert_block(then_block);
                builder.jump(end_block, then_args);
                builder.set_insert_block(else_block);
                builder.jump(end_block, else_args);
                builder.set_insert_block(end_block);
                let mut end_args = Vec::new();
                for i in 0..block_param_types.len() {
                    end_args
                        .push(Value::from(builder.get_block_param(end_block, i as u32)));
                }
                let mut consumed_total = 0;
                let packable: lift::types::AirPackable = result.clone().into();
                let (packed, consumed) = packable
                    .pack_from_air_values_and_types(
                        &end_args[consumed_total..],
                        &block_param_types[consumed_total..],
                    )?;
                result = packed.try_into()?;
                consumed_total += consumed;
            }
            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    {
        let arg_0 = {
            let arg_0 = lift::types::Variable::from(common::types::integer::from(0));
            let arg_1 = {
                let arg_0 = {
                    let arg_0 = {
                        let arg_0 = destsize.clone();
                        let arg_1 = lift::types::Variable::from(
                            common::types::integer::from(1),
                        );
                        lift::helpers::sub_int_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                        )?
                    };
                    let arg_1 = lift::types::Variable::from(
                        common::types::integer::one(),
                    );
                    lift::helpers::add_int_0(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
                        arg_1,
                    )?
                };
                let arg_1 = lift::types::Variable::from(common::types::integer::from(0));
                lift::helpers::sub_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            result.extract_slice(builder, arg_0, arg_1)?
        };
        let arg_1 = d.clone();
        let arg_2 = {
            let arg_0 = lift::types::Variable::from(common::types::integer::from(0));
            let arg_1 = {
                let arg_0 = {
                    let arg_0 = {
                        let arg_0 = destsize.clone();
                        let arg_1 = lift::types::Variable::from(
                            common::types::integer::from(1),
                        );
                        lift::helpers::sub_int_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                        )?
                    };
                    let arg_1 = lift::types::Variable::from(
                        common::types::integer::from(0),
                    );
                    lift::helpers::sub_int_0(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
                        arg_1,
                    )?
                };
                let arg_1 = lift::types::Variable::from(common::types::integer::from(1));
                lift::helpers::add_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            lift::helpers::add_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
        };
        lift::helpers::X_set_0(builder, sequencer, pc.clone(), arg_0, arg_1, arg_2)?;
    }
    Ok(())
}
