use super::*;

const WASM_OP: WasmOp = WasmOp::binary(WasmType::I32, "mul");

#[test]
#[cfg_attr(miri, ignore)]
fn reg_reg() {
    test_binary_reg_reg(WASM_OP, Instruction::i32_mul)
}

#[test]
#[cfg_attr(miri, ignore)]
fn reg_imm16() {
    test_binary_reg_imm16::<i32>(WASM_OP, 100, Instruction::i32_mul_imm16)
}

#[test]
#[cfg_attr(miri, ignore)]
fn reg_imm16_rev() {
    test_binary_reg_imm16_rev::<i32>(WASM_OP, 100, swap_ops!(Instruction::i32_mul_imm16))
}

#[test]
#[cfg_attr(miri, ignore)]
fn reg_imm() {
    test_binary_reg_imm32(WASM_OP, i32::MAX, Instruction::i32_mul)
}

#[test]
#[cfg_attr(miri, ignore)]
fn reg_imm_rev() {
    test_binary_reg_imm32_rev_commutative(WASM_OP, i32::MAX, Instruction::i32_mul)
}

#[test]
#[cfg_attr(miri, ignore)]
fn reg_zero() {
    let expected = [Instruction::ReturnImm32 {
        value: AnyConst32::from_u32(0),
    }];
    test_binary_reg_imm_with(WASM_OP, 0_i32, expected).run()
}

#[test]
#[cfg_attr(miri, ignore)]
fn reg_zero_rev() {
    let expected = [Instruction::ReturnImm32 {
        value: AnyConst32::from_u32(0),
    }];
    test_binary_reg_imm_rev_with(WASM_OP, 0_i32, expected).run()
}

#[test]
#[cfg_attr(miri, ignore)]
fn reg_one() {
    let expected = [Instruction::return_reg(0)];
    test_binary_reg_imm_with(WASM_OP, 1_i32, expected).run()
}

#[test]
#[cfg_attr(miri, ignore)]
fn reg_one_rev() {
    let expected = [Instruction::return_reg(0)];
    test_binary_reg_imm_rev_with(WASM_OP, 1_i32, expected).run()
}

#[test]
#[cfg_attr(miri, ignore)]
fn consteval() {
    let lhs = 1;
    let rhs = 2;
    test_binary_consteval(
        WASM_OP,
        lhs,
        rhs,
        [Instruction::ReturnImm32 {
            value: AnyConst32::from_i32(lhs * rhs),
        }],
    )
}
