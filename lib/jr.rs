#[derive(Clone)]
pub struct JRCodeInfo {
	inst: String,
	opcode: String,
	func_code: String,
}

impl JRCodeInfo {
	pub fn new(inst: String, opcode: String, func_code: String) -> JRCodeInfo {
		JRCodeInfo {
			inst: inst,
			opcode: opcode,
			func_code: func_code,
		}
	}

	pub fn get_func_code(&self) -> String {
		self.func_code.clone()
	}

	pub fn get_opcode(&self) -> String {
		self.opcode.clone()
	}
}

pub fn gen_jr_objs() -> Vec<(String, JRCodeInfo)> {
	let mut tmpVec: Vec<(String, JRCodeInfo)> = Vec::new();
	let inst = ["add","addu","and","div","divu"
	,"mfhi","mflo","mfc0","mult","multu","nor","xor","or"
	,"slt","sltu","sll","srl","sra","sub","subu"];
	let op = ["0x0","0x0","0x0","0x0","0x0","0x0"
	,"0x0","0x10","0x0","0x0","0x0","0x0"
	,"0x0","0x0","0x0","0x0", "0x0","0x0"];
//	let func_code = [];

	for i in 0..inst.len() {

		
	}
	return tmpVec
}