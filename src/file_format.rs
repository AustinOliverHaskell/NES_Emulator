pub struct INES {
	pub header: INESHeader,
	pub trainer: Option<[u8; 512]>, 
	pub program_rom: Vec<u8>,
	pub char_rom: Vec<u8>, 
	pub play_choice_inst_rom: Vec<u8>,
	pub play_choice_prom: Vec<u8>,
	pub title: Option<Vec<u8>>
}

pub struct INESHeader {
	pub nes_magic_present: bool,
	pub program_rom_size: u8, 
	pub char_rom_size: u8,
	pub flag_group_6: u8,
	pub flag_group_7: u8, 
	pub flag_group_8: u8,
	pub flag_group_9: u8,
	pub flag_group_10: u8,
	pub extra_padding: [u8; 5]
}

impl INESHeader {
	pub fn from_bytes(bytes: &[u8]) -> Self {

		println!("Got header of [{:?}]", bytes);

		let nes_magic_present = 
			bytes[0] == 'N' as u8 && 
			bytes[1] == 'E' as u8 && 
			bytes[2] == 'S' as u8 &&
			bytes[3] == 26;

		Self {
			nes_magic_present: nes_magic_present,
			program_rom_size: bytes[4],
			char_rom_size: bytes[5],
			flag_group_6: bytes[6],
			flag_group_7: bytes[7],
			flag_group_8: bytes[8],
			flag_group_9: bytes[9],
			flag_group_10: bytes[10],
			extra_padding: [bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]]
		}
	}

	pub fn is_trainer_present(self: &Self) -> bool {
		(self.flag_group_6 & 0x04) != 0
	} 
}

impl INES {
	pub fn from_bytes(bytes: Vec<u8>) -> Self {

		println!("Got {:} bytes for ines file. ", bytes.len());

		let header = INESHeader::from_bytes(&bytes[0..16]);

		let trainer_offset = if header.is_trainer_present() { 512 } else { 0 };

		let prog_offsets: (usize, usize) = (trainer_offset + 16, 16384 * header.program_rom_size as usize + trainer_offset);
		let char_offsets: (usize, usize) = (prog_offsets.1, 8192 * header.char_rom_size as usize + prog_offsets.1);

		let program_rom = bytes[prog_offsets.0..prog_offsets.1].to_vec(); 
		let char_rom = if header.char_rom_size != 0 { bytes[char_offsets.0..char_offsets.1].to_vec() } else { Vec::new() }; 

		println!("--------------- PROGRAM ROM ---------------");
		crate::util::print_memory_block(&program_rom, 16, true, (trainer_offset + 16) as u32, true);
		println!("------------ END OF PROGRAM ROM ------------");
			println!("Trainer is {:} present", if header.is_trainer_present() {""} else {"NOT"});

		Self {
			header: header, 
			trainer: None,
			program_rom: program_rom,
			char_rom: char_rom,
			play_choice_inst_rom: Vec::new(),
			play_choice_prom: Vec::new(),
			title: None
		}
	}
}