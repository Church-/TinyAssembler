pub struct ImmCodeInfo<'a> {
	inst: &'a str,
	opcode: &'a str,
	func_code: &'a str,
}

impl<'a> ImmCodeInfo<'a> {
	pub	fn new(inst: &'a str, opcode: &'a str, func_code: &'a str) -> ImmCodeInfo<'a> {
		ImmCodeInfo {
			inst: inst,
			opcode: opcode,
			func_code: func_code,
		}
	}

	pub	fn get_func_code(&self) -> &'a str {
		return self.func_code
	}

	pub fn get_opcode(&self) -> &'a str {
		return self.opcode
	}
}

fn gen_objs<'a>() -> (&'a str, a'ImmCodeInfo) {
	let inst = ["add","addu","and","div","divu"
	,"mfhi","mflo","mfc0","mult","multu","nor","xor","or"
	,"slt","sltu","sll","srl","sra","sub","subu"];
	let op = ["0x0","0x0","0x0","0x0","0x0","0x0"
	,"0x0","0x10","0x0","0x0","0x0","0x0"
	,"0x0","0x0","0x0","0x0", "0x0","0x0"];
	let func_code = [];

	for i in inst.len() {

		
	}
}