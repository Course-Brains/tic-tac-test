use std::io;

struct Out {
	valid: bool,
	x: usize,
	y: usize
}
struct Board {
	board: [[String; 3]; 3],
	turn: String
}
impl Board {
	fn new() -> Board{
		return Board {
			board:[
				[String::from(" "),String::from(" "),String::from(" ")],
				[String::from(" "),String::from(" "),String::from(" ")],
				[String::from(" "),String::from(" "),String::from(" ")]
			],
			turn: String::from("o")
		}
	}
	fn tie_check(&self) -> bool {
		//returns true if tie, checks all spaces to see if all are filled, if it finds a space it is not a tie yet
		for row in self.board.iter() {
			for i in row.iter() {
				if i == &String::from(" ") {
					return false;
				}
			}
		}
		return true
	}
	fn draw(&self){
		let mut draw_connector:bool = false;
		println!("    0   1   2");
		println!("  ╔═══╦═══╦═══╗");
		for (index,row) in self.board.iter().enumerate() {
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
	fn end_check(&self) -> Out {
		//if this function returns false then the game continues
		let mut valid:bool = true;
		let mut check:[usize;2] = [0,0];
		for (index, row) in self.board.iter().enumerate() {
			if !(row[1] == " "){// if even one is a space then it fails, allows us to not have to check the others because even if it makes it past this check it will be caught by the main check
				if row[0] == row[1] && row[1] == row[2] {
					valid = false;
					check = [index.try_into().unwrap(),1];
				}
			}
		}
		for i in 0..=2 {
			if !(self.board[0][i] == " "){
				if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] {
					valid = false;
					check = [1,i.try_into().unwrap()];
				}
			}
		}
		if !(self.board[1][1] == " "){// only need one check because both pass through [1][1]
			if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
				valid = false;
				check = [1,1];
			}
			else if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
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
	fn game_loop(mut self){
		// note to self: move board creation to this
		loop {
			if self.tie_check() {
				println!("tie");
				break
			}
			if self.turn == String::from("x") {
				println!("O turn");
				self.turn = String::from("o")
			}
			else {
				println!("X turn");
				self.turn = String::from("x")
			}
			self.draw();
			let temp: String = prompt();
			let input: [String; 2] = [
				temp.chars().nth(1).unwrap_or_default().to_string(),
				temp.chars().nth(0).unwrap_or_default().to_string()
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
			if self.board[input[0]][input[1]] == " " {
				//valid input
				self.board[input[0]][input[1]] = self.turn.clone();
			}
			else {
				//it is swapped at the start so this makes it so that it ends up not changing
				if self.turn == String::from("x"){
					self.turn = String::from("o");
				}
				else {
					self.turn = String::from("x")
				}
				continue
			}
			let end: Out = self.end_check();
			if end.valid {
				continue
			}
			let winner:&String = &self.board[end.x][end.y];
			self.draw();
			println!("Team {} won!", &winner);
			break
		}
	}
}
fn prompt() -> String {
	let mut input:String = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("failed to read line");
	return input
}
fn main(){
	let game:Board = Board::new();
	game.game_loop();
}