#[derive(Clone)]
pub struct ImmCodeInfo {
	inst: String,
	opcode: String,
	func_code: String,
}

impl ImmCodeInfo {
	pub	fn new(inst: String, opcode: String, func_code: String) -> ImmCodeInfo {
		ImmCodeInfo {
			inst: inst,
			opcode: opcode,
			func_code: func_code,
		}
	}

	pub	fn get_func_code(&mut self) -> String {
		return self.func_code.clone()
	}

	pub fn get_opcode(&self) -> String {
		return self.opcode.clone()
	}
}

pub fn gen_imm_objs() -> Vec<(String, ImmCodeInfo)> {
	let tmpVec: Vec<(String, ImmCodeInfo)> = Vec::new();
	let inst = ["add","addu","and","div","divu"
	,"mfhi","mflo","mfc0","mult","multu","nor","xor","or"
	,"slt","sltu","sll","srl","sra","sub","subu"];
	let op = ["0x0","0x0","0x0","0x0","0x0","0x0"
	,"0x0","0x10","0x0","0x0","0x0","0x0"
	,"0x0","0x0","0x0","0x0", "0x0","0x0"];

	for i in 0..inst.len() {
		
	}
	return tmpVec
}