use std::collections::HashMap;
use lib::reg::{gen_reg_objs, gen_reg_codes,RegCodeInfo};
use lib::jr::{gen_jr_objs, JRCodeInfo};
use lib::shift_reg::{gen_shift_objs, ShiftRegCodeInfo};
use lib::jump::{gen_jump_objs, JumpCodeInfo};
use lib::imm::{gen_imm_objs, ImmCodeInfo};

pub struct CodeInfo {
	pub reg_codes: HashMap<String, RegCodeInfo>,
	pub jr_code: HashMap<String, JRCodeInfo>,
	pub shift_reg_codes: HashMap<String, ShiftRegCodeInfo>,
	pub jump_codes: HashMap<String, JumpCodeInfo>,
	pub imm_codes: HashMap<String, ImmCodeInfo>,
	pub regs: HashMap<String,String>,
	pub labels: HashMap<String, usize>,
	pub line_num: usize,
}

impl CodeInfo {

	pub fn new() -> CodeInfo {
		let reg_codes_vec = gen_reg_objs();
		let jr_code_vec = gen_jr_objs();
		let shift_reg_codes_vec = gen_shift_objs();
		let imm_codes_vec = gen_imm_objs();
		let jump_codes_vec = gen_jump_objs();
		let regs_vec = gen_reg_codes();
		
		CodeInfo {
			reg_codes: reg_codes_vec.into_iter().clone().collect(),
			jr_code: jr_code_vec.into_iter().clone().collect(),
			shift_reg_codes: shift_reg_codes_vec.into_iter().clone().collect(),
			imm_codes: imm_codes_vec.into_iter().clone().collect(),
			jump_codes: jump_codes_vec.into_iter().clone().collect(),
			regs: regs_vec.into_iter().clone().collect(),
			labels: HashMap::new(),
			line_num: 0,
		}
	}
}
