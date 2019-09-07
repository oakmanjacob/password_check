use std::env;

// Characters which can't be typed via a traditional keyboard have been omitted
static screen: [&String; 5] = [
	String::from("abcdefghijklmnopqrstuvwxyz1234567890,. "),
	String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890,. "),
	String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890,. "),
	String::from("+=/_<>[]!@#$%^&*()-'\":;,?1234567890,. "), 
	String::from("`~\\|{}1234567890,. ")
];

fn main() {
	let args: Vec<String> = env::args().collect();

	let mut curscreen: u8;
	let mut screen_change_counter: u16;
	let mut button_press_counter: u16;
	let value_chars = &args[1].chars();

	curscreen = 0;
	let mut index: usize = 0;

	while value_chars.hasNext() {
		if screen[curscreen].contians(c);

		value_chars.next();
	}
}