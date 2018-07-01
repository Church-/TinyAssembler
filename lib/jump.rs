#[derive(Clone)]
pub struct JumpCodeInfo {
	inst: String,
	opcode: String,
	func_code: String,
}

impl JumpCodeInfo {
	pub fn new(inst: String, opcode: String, func_code: String) -> JumpCodeInfo {
		JumpCodeInfo {
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

pub fn gen_jump_objs() -> Vec<(String, JumpCodeInfo)> {
	let mut tmpVec: Vec<(String, JumpCodeInfo)> = Vec::new();
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