use std::collections::HashMap;

fn main() {
	// let _args: Vec<String> = env::args().collect();
	// let password_chars = &args[1].chars();
	let password = "Sol3Rest&$Kwl".chars();

	let screen_str: [String; 4] = [
		String::from("abcdefghijklmnopqrstuvwxyz1234567890,. "),
		String::from("+=/_<>[]!@#$%^&*()-'\":;,?1234567890,. "),
		String::from("`~\\|{}1234567890,. "),
		String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890,. ")
	];

	let mut char_list : HashMap<char, Vec<usize>> = HashMap::new();

	for scr in 0..4 {
		for c in screen_str[scr].chars() {
			match char_list.get_mut(&c.clone()) {
				Some(vector) => {
					vector.push(scr.clone());
				},
				None => {char_list.insert(c.clone(), vec![scr.clone()]); ()}
			};
			println!("{}", scr.clone());
		} 
	}

	println!("Added");

	let mut cur_screen = 3;
	let mut cost = 0;

	for c in password {
		match char_list.get(&c.clone())
		{
			Some(vector) => {
				let mut min_cost = 10;
				let mut next_screen = 0;
				for i in vector {
					let switch_cost = get_switch_cost(cur_screen.clone(), i.clone());

					if min_cost > switch_cost {
						min_cost = switch_cost;
						next_screen = i.clone();
					}
					print!("{} ", i);
				}
				println!("");

				cost += min_cost;
				cur_screen = next_screen;
			},
			None => panic!("Character not found!")
		};
	}

	println!("{}", cost);
}

fn get_switch_cost(screen1: usize, screen2: usize) -> usize {
	match screen1 {
		0 => match screen2 {
			0 => 0,
			1 => 1,
			2 => 2,
			_ => 0
		},
		1 => match screen2 {
			0 => 2,
			1 => 0,
			2 => 1,
			_ => 0
		},
		2 => match screen2 {
			0 => 1,
			1 => 2,
			2 => 0,
			_ => 0
		},
		_ => 0
	}
}