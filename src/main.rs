use std::io;

fn draw(map: &[[String; 3]; 3]){
	let mut draw_connector:bool = false;
	println!("    0   1   2");
	println!("  ╔═══╦═══╦═══╗");
	for (index,row) in map.iter().enumerate() {
		if draw_connector{
			println!("  ╠═══╬═══╬═══╣");
		}
		else{
			draw_connector = true;
		}
		println!("{} ║ {} ║ {} ║ {} ║",index,row[0],row[1],row[2])
	}
	println!("  ╚═══╩═══╩═══╝");
}
struct Out {
	valid: bool,
	x: usize,
	y: usize
}
fn prompt() -> String {
	let mut input:String = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("failed to read line");
	return input
}
fn end_check(map: &[[String; 3]; 3]) -> Out {
	//if this function returns false then the game continues
	let mut valid:bool = true;
	let mut check:[usize;2] = [0,0];
	for (index, row) in map.iter().enumerate() {
		if !(row[1] == " "){// if even one is a space then it fails, allows us to not have to check the others because even if it makes it past this check it will be caught by the main check
			if row[0] == row[1] && row[1] == row[2] {
				valid = false;
				check = [index.try_into().unwrap(),1];
			}
		}
	}
	for i in 0..=2 {
		if !(map[0][i] == " "){
			if map[0][i] == map[1][i] && map [1][i] == map[2][i] {
				valid = false;
				check = [1,i.try_into().unwrap()];
			}
		}
	}
	if !(map[1][1] == " "){// only need one check because both pass through [1][1]
		if map[0][0] == map[1][1] && map[1][1] == map[2][2] {
			valid = false;
			check = [1,1];
		}
		else if map[0][2] == map[1][1] && map[1][1] == map [2][0] {
			valid = false;
			check = [1,1];
		}
	}
	// team determination
	if valid {
		return Out {
			valid: true,
			x: 0,
			y: 0
		}
	}
	return Out {
		valid: false,
		x: check[0],
		y: check[1]
	}

	
}
fn tie_check(board: &[[String; 3]; 3]) -> bool {
	//returns true if tie, checks all spaces to see if all are filled, if it finds a space it is not a tie yet
	for row in board.iter() {
		for i in row.iter() {
			if i == &String::from(" ") {
				return false;
			}
		}
	}
	return true
}
fn game_loop(board: &mut[[String; 3]; 3]){
    // note to self: move board creation to this
	let mut turn: String = String::from("o");
	loop {
		if tie_check(board) {
			println!("tie");
			break
		}
		if turn == String::from("x") {
			println!("O turn");
			turn = String::from("o")
		}
		else {
			println!("X turn");
			turn = String::from("x")
		}
		draw(&board);
		let temp: String = prompt();
		let input: [String; 2] = [
			temp.chars().nth(0).unwrap_or_default().to_string(),
			temp.chars().nth(1).unwrap_or_default().to_string()
		];
		let mut temp: [usize; 2] = [4,4];
		for i in 0..=1 {
			match input[i].parse::<usize>() {
				Ok(number) => {
					if number > 2 {
						println!("number out of range")
					}
					temp[i] = number;
				}
				Err(err) => {
					println!("unable to parse to usize: {}",err)
				}
			}
		}
		let input: [usize; 2] = temp;
		// bad input handling
		if board[input[0]][input[1]] == " " {
			//valid input
			board[input[0]][input[1]] = turn.clone();
		}
		else {
			//it is swapped at the start so this makes it so that it ends up not changing
			if turn == String::from("x"){
				turn = String::from("o");
			}
			else {
				turn = String::from("x")
			}
			continue
		}
		let end: Out = end_check(&board);
		if end.valid {
			continue
		}
		let winner:&String = &board[end.x][end.y];
		println!("Team {} won!", &winner);
		break
	}
	
}
fn reset() -> [[String; 3]; 3] {
	let board: [[String; 3]; 3] = [
		[String::from(" "),String::from(" "),String::from(" ")],
		[String::from(" "),String::from(" "),String::from(" ")],
		[String::from(" "),String::from(" "),String::from(" ")]
	];
	return board
}
fn main(){
	game_loop(& mut reset())
}