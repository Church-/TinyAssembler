use std::collections::HashMap;
use lib::reg::RegCodeInfo;
use lib::jr::JRCodeInfo;
use lib::shift_reg::ShiftRegCodeInfo;
use lib::jump::JumpCodeInfo;
use lib::imm::ImmCodeInfo;

pub struct CodeInfo<'a> {
	pub reg_codes: HashMap<&'a str, RegCodeInfo>,
	pub jr_code: HashMap<&'a str, JRCodeInfo>,
	pub shift_reg_codes: HashMap<&'a str, ShiftRegCodeInfo>,
	pub jump_codes: HashMap<&'a str, JumpCodeInfo>,
	pub imm_codes: HashMap<&'a str, ImmCodeInfo>,
	pub regs: HashMap<&'a str, &'a str>,
	pub labels: HashMap<&'a str, usize>,
	pub line_num: usize,
}

impl<'a> CodeInfo<'a> {

	pub fn new() -> CodeInfo<'a> {
		CodeInfo {
			reg_codes: [("A","B")].iter().cloned().collect(),
			jr_code: [("","")].iter().cloned().collect(),
			shift_reg_codes: [("","")].iter().cloned().collect(),
			imm_codes: [("A","B")].iter().cloned().collect(),
			jump_codes: [("A","B")].iter().cloned().collect(),
			regs: [("A","B")].iter().cloned().collect(),
			labels: HashMap::new(),
			line_num: 0,
		}
	}
}
