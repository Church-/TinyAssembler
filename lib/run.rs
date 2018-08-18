//extern crate rand
//extern crate failure;

use std::str::Lines;
use std::str;
use std::env::args;
use std::error::Error;
use std::io::Read;
use std::io::prelude;
use std::fs::File;
// use self::failure::Error;
//use rand::Rng;
pub use lib::asm_info::CodeInfo;

fn convert_line(line: Vec<String>, mut info: &CodeInfo) -> Result<String, Box<Error>> {
	let mut tmpBuf = String::new();
	match info.jump_codes.contains_key(&line[0]) {
		true => {
			if line.len() > 2 { 
				panic!("Incorrect number of arguments. Check line #{}",info.line_num); 
			}
			else {
				tmpBuf.push_str(&info.jump_codes.get(&line[0])
				.expect("Failed to get value").get_func_code());
				tmpBuf.push_str(&line[1].split("0x")
				.nth(1).expect("Failed to get value"));
				}
			},
			_ => (),
		}

	match info.jr_codes.contains_key(&line[0]) {
		true => {
			if line.len() != 2 { 
				panic!("Incorrect number of arguments. Check line #{}",info.line_num); 
			}
			else {
				tmpBuf.push_str(&info.reg_codes.get(&line[0])
				.expect("Failed to get value").get_opcode());//Jump Reg Opcode
				tmpBuf.push_str(&info.regs.get(&line[1])
				.expect("Failed to get value"));//Reg
				tmpBuf.push_str("");//Two other regs and shift amount
				tmpBuf.push_str(&info.reg_codes.get(&line[0])
				.expect("Failed to get value").get_func_code());//Func Code
			}		
		},
		_ => (),
	}


	match info.shift_reg_codes.contains_key(&line[0]) {
		true => {
			if line.len() != 4 { 
				panic!("Incorrect number of arguments. Check line #{}",info.line_num); 
			}
			else {
				tmpBuf.push_str(&info.regs.get(&line[1])
				.expect("Failed to get value"));//Reg
				tmpBuf.push_str(&info.regs.get(&line[2])
				.expect("Failed to get value"));//reg
				tmpBuf.push_str("00000");// Zero'd out reg
				tmpBuf.push_str(&line[3]);//Shift Amount
				tmpBuf.push_str(&info.reg_codes.get(&line[0])
				.expect("Failed to get value").get_func_code());//Func Code
			}
		},
		_ => (),
	}
	
	match info.reg_codes.contains_key(&line[0]) {
		true => {
			if line.len() != 4 { 
				panic!("Incorrect number of arguments. Check line #{}",info.line_num); 
			}
			else {
				tmpBuf.push_str(&info.reg_codes.get(&line[0])
				.expect("Failed to get value").get_opcode());//Most other OP Codes
				line.iter()
					.skip(1)
					.map(|x| 
						match info.regs.contains_key(x) {
							true => { 
									line.iter()
										.skip(1)
										.map(|x| tmpBuf.push_str(&info.regs.get(x)
										.expect("Failed to get value.")));

									tmpBuf.push_str("00000");//Shift Amount
									tmpBuf.push_str(&info.reg_codes.get(&line[0])
									.expect("Failed to get value").get_func_code());//Func Code
								},
							_ => panic!("Incorrect arguments. Check line #{}",info.line_num),
				});

				}
			},

			_ => (),
		}

	match info.imm_codes.contains_key(&line[0]) {
		true => {},
		_ => (),
		}
	
	
/*	match info.pseudo_opcodes.contains_key(&line[0]) {
				
		}*/

	Ok(tmpBuf)
}



fn assemble_loop(prog_lines: Lines, mut info: CodeInfo) -> Result<String, Box<Error>> {
	let mut address = 10000000; 
	let mut tmpBuf = String::new();
	for i in prog_lines {

		info.line_num += 1;
		address += 4;

		let mut tmp_line = i.split(" ").collect::<Vec<&str>>();
		let mut line: Vec<String> = tmp_line.iter()
											.map(|n| n.to_string())
											.collect::<Vec<String>>();

		match line[0].contains(":") {
			true => {
				info.labels.insert(line[0].split(":")
										  .nth(0)
										  .expect("Failed to get value")
										  .to_string(),address);

				if !(line[0].split(":").nth(1).unwrap().to_string().is_empty()) {

					line[0] = line[0].split(":").nth(1)
					.expect("Failed to get value").to_string();

					tmpBuf.push_str(&convert_line(line.clone(),&info)?);
				}
			},
			_ => continue,
		}

		tmpBuf.push_str(&convert_line(line,&info)?);
	}
	
	Ok(tmpBuf)
}

pub fn run() -> Result<(),()> {
	let info = CodeInfo::new();
    let mut buf = String::new();
    let mut arg: Vec<String> = args().collect::<_>();
    let mut f = File::open(&arg[1])
    .expect("Failed to open file");
    f.read_to_string(&mut buf).expect("Failed to write to buffer");
    //let prog_lines = buf.split("\n").collect::<Vec<_>>();
	let prog_lines = buf.lines();
	let assembly = assemble_loop(prog_lines,info).unwrap();
	println!("{}",assembly);
	Ok(())
}
