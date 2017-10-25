use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kandw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDW, operand1: Some(Direct(K5)), operand2: Some(Direct(K7)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 65, 235], OperandSize::Dword)
}

#[test]
fn kandw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDW, operand1: Some(Direct(K7)), operand2: Some(Direct(K6)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 65, 251], OperandSize::Qword)
}
