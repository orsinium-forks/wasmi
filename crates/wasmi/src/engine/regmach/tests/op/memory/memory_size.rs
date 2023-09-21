use super::*;

#[test]
#[cfg_attr(miri, ignore)]
fn reg() {
    let wasm = wat2wasm(
        r"
        (module
            (memory $m 10)
            (func (result i32)
                (memory.size $m)
            )
        )",
    );
    TranslationTest::new(wasm)
        .expect_func_instrs([
            Instruction::memory_size(Register::from_i16(0)),
            Instruction::return_reg(Register::from_i16(0)),
        ])
        .run();
}
