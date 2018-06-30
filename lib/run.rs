//extern crate rand;

use std::str::Lines;
use std::str;
use std::io::Read;
use std::io::prelude;
use std::fs::File;
//use rand::Rng;
use lib::CodeInfo;

fn convert_line(line: Vec<&str>, mut info: &CodeInfo) -> String {
	let mut tmpBuf = String::new();
	match info.jump_codes.contains_key(line[0]) {
		true => {
			if line.len() > 2 { 
				panic!("Incorrect number of arguments. Check line #{}",info.line_num); 
			}
			else {
				tmpBuf.push_str(info.jump_codes.get(line[0]).unwrap());
				tmpBuf.push_str((line[1].split("0x").nth(1).unwrap()));
				}
			},
			_ => (),
		}

	match info.jr_code.contains_key([0]) {
		true => {
			if line.len() != 2 { 
				panic!("Incorrect number of arguments. Check line #{}",info.line_num); 
			}
			else {
				tmpBuf.push_str(info.reg_codes.get(line[0]).get_opcode());//Jump Reg Opcode
				tmpBuf.push_str(info.regs.get(line[1]));//Reg
				tmpBuf.push_str("");//Two other regs and shift amount
				tmpBuf.push_str(info.reg_codes.get(line[0]).get_func_code());//Func Code
			}		
		},
		_ => (),
	}


	match info.shift_code.contains_key(line[0]) {
		true => {
			if line.len() != 4 { 
				panic!("Incorrect number of arguments. Check line #{}",info.line_num); 
			}
			else {
				tmpBuf.push_str(info.regs.get(line[1]));//Reg
				tmpBuf.push_str(info.regs.get(line[2]));//reg
				tmpBuf.push_str("00000");// Zero'd out reg
				tmpBuf.push_str(line[3]);//Shift Amount
				tmpBuf.push_str(info.reg_codes.get(line[0]).get_func_code());//Func Code
			}
		},
		_ => (),
	}
	
	match info.reg_codes.contains_key(line[0]) {
		true => {
			if line.len() != 4 { 
				panic!("Incorrect number of arguments. Check line #{}",info.line_num); 
			}
			else {
				tmpBuf.push_str(info.reg_codes.get(line[0]).get_opcode());//Most other OP Codes
				line.iter()
					.skip(1)
					.map(|x| 
						match info.regs.contains_key(x) {
							true => { 
									line.iter()
										.skip(1)
										.map(|x| tmpBuf.push_str(info.regs.get(x)));

									tmpBuf.push_str("00000");//Shift Amount
									tmpBuf.push_str(info.reg_codes.get(line[0]).get_func_code());//Func Code
								},
							_ => panic!("Incorrect arguments. Check line #{}",info.line_num),
				});

				}
			},

			_ => (),
		}

	match info.imm_codes.contains_key(line[0]) {
		true => {},
		_ => (),
		}
	
	
/*	match info.pseudo_opcodes.contains_key(line[0]) {
				
		}*/

	return tmpBuf
}



fn assemble_loop<'a>(prog_lines: std::str::Lines<'a>,mut info: CodeInfo<'a>) -> String {
	let mut address = 10000000; 
	let mut tmpBuf = String::new();
	for i in prog_lines {
		info.line_num += 1;
		address += 4;
		let mut line = i.split(" ").collect::<Vec<_>>();
		match line[0].contains(":") {
			true => {
				info.labels.insert(line[0].split(":").nth(0).unwrap(),address);
				if !(line[0].split(":").nth(1).unwrap().to_string().is_empty()) {
					line[0] = line[0].split(":").nth(1).unwrap();
					tmpBuf.push_str(&convert_line(line.clone(),&info));
				}
			},
			_ => continue,
		}

		tmpBuf.push_str(&convert_line(line,&info));
	}
	
	return tmpBuf
}

pub fn run() {
	let info = CodeInfo::new();
    let mut buf = String::new();
    let mut arg: Vec<String> = std::env::args().collect::<_>();
    let mut f = File::open(&arg[1])
    .expect("Failed to open file");
    f.read_to_string(&mut buf).expect("Failed to write to buffer");
    //let prog_lines = buf.split("\n").collect::<Vec<_>>();
	let prog_lines = buf.lines();
	assemble_loop(prog_lines,info);
}