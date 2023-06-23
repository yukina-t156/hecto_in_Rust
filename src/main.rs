use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;


fn to_ctrl_byte(c: char) -> u8 {
	let byte = c as u8;
	// ctrl characters have the upper 3 bits set to 0.
	// so, if byte is ctrl character, byte & 0b0001_1111 is same as byte.
	byte & 0b0001_1111
}

fn main() {
	let _stdout = stdout().into_raw_mode().unwrap();
	
    // キーボードの入力をbにバインド
    for b in io::stdin().bytes() {
	    // １文字ずつ出力
		let b = b.unwrap();
		let c = b as char;
		if c.is_control() {
			println!("{:?} \r", b);
		} else {
			println!("{:?} ({:?})\r", b, c);
		}
		// cltr-Q can't be used in VSCode
	    if b == to_ctrl_byte('c') {
		    break;
	    }
	}
}
