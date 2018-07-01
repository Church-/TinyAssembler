#[derive(Clone)]
pub struct RegCodeInfo {
	inst: String,
	opcode: String,
	func_code: String,
}

impl RegCodeInfo {
	pub fn new(inst: String, opcode: String, func_code: String) -> RegCodeInfo {
		RegCodeInfo {
			inst: inst,
			opcode: opcode,
			func_code: func_code,
		}
	}

	pub fn get_func_code(&self) -> String {
		return self.func_code
	}

	pub fn get_opcode(&self) -> String {
		return self.opcode
	}
}

pub fn gen_reg_codes() -> Vec<(String, String)> {
	let mut tmpVec: Vec<(String, String)> = Vec::new();
	let inst = ["add","addu","and","div","divu"
	,"mfhi","mflo","mfc0","mult","multu","nor","xor","or"
	,"slt","sltu","sll","srl","sra","sub","subu"];
	let op = ["0x0","0x0","0x0","0x0","0x0","0x0"
	,"0x0","0x10","0x0","0x0","0x0","0x0"
	,"0x0","0x0","0x0","0x0", "0x0","0x0"];
	let func_code = [];

	for i in 0..inst.len() {

		
	}
	return tmpVec
}


pub fn gen_reg_objs() -> Vec<(String, RegCodeInfo)> {
	let mut tmpVec: Vec<(String, RegCodeInfo)> = Vec::new();
	let inst = ["add","addu","and","div","divu"
	,"mfhi","mflo","mfc0","mult","multu","nor","xor","or"
	,"slt","sltu","sll","srl","sra","sub","subu"];
	let op = ["0x0","0x0","0x0","0x0","0x0","0x0"
	,"0x0","0x10","0x0","0x0","0x0","0x0"
	,"0x0","0x0","0x0","0x0", "0x0","0x0"];
	let func_code = [];

	for i in 0..inst.len() {

		
	}
	return tmpVec
}