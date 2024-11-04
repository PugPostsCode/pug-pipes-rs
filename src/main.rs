/*
	Draw pipes into the terminal
*/
use std::io;
use std::time::Duration;

use std::env::args;

use rand::prelude::random;

use crossterm::{
	ExecutableCommand,
	cursor::*,
	terminal::*,
	event::*,
	event::KeyCode
};

// Structs
#[derive(Copy, Clone)]
struct Pipe {
	x_pos: u16,
	y_pos: u16,
	direction: u8
}
impl Pipe {
	// create new pipe
	fn new(x_pos: u16, y_pos: u16, direction: u8) -> Self {
		Pipe{
			x_pos,
			y_pos,
			direction
		}
	}
}

// Main
fn main() {
	// default values
	let mut pipe_count = 4;
	let mut update_speed = 10;
	let mut reset_cycles = 1000;
	let mut show_cycles = false;
	let mut help = false;

	// parse args
	let mut next = "";
	for arg in args() {
		// values
		if next == "pipes" {
			pipe_count = arg.parse().expect("Please input a number!");
			next = "";
		}
		else if next == "speed" {
			update_speed = arg.parse().expect("Please input a number!");
			next = "";
		}
		else if next == "reset" {
			reset_cycles = arg.parse().expect("Please input a number!");
			next = "";
		}
	
		// flags
		if arg == "-p" {
			next = "pipes";
		}
		else if arg == "-s" {
			next = "speed";
		}
		else if arg == "-r" {
			next = "reset"
		}
		else if arg == "-c" {
			show_cycles = true;
		}
		else if arg == "-h" || arg == "--help" {
			help = true;
		}
	}
	
	// run or display help
	if help {
		// print a help message
		println!("Flags:\n\t-p [number]: amount of pipes to simulate\n\t-s [number]: amount of miliseconds between \"frames\"\n\t-r [number]: amount of cycles until the simulation resets\n\t-c: shows the amount of cycles passed since last reset\n\t-h: show this help message");
	}
	else {
		// run the pipes
		run_pipes(
			pipe_count,
			update_speed,
			reset_cycles,
			show_cycles
		);
	}

	
}

// run the pipes
fn run_pipes(pipe_count: u64, update_speed: u64, reset_cycles: u64, show_cycles: bool) {
	// get dimenstions
	let (width, height) = size().unwrap();

	// setup a grid of width x height
	let mut grid: Vec::<Vec<u8>> = Vec::new();

	// populate grid
	for x in 0..width {
		// get a usize of x
		let index_x: usize = x.into();

		// push empty Vec
		grid.push(
			Vec::new()
		);

		// loop y
		for y in 0..(height - 3) { // how is this unused it's used for the for loop????????
			// get a usize of y
			let index_y: usize = y.into();

			// set value in grid
			grid[index_x].push(
				0
			);

			//
			grid[index_x][index_y] = 0;
		}
	}

	// setup pipes
	let mut pipes: Vec<Pipe> = Vec::new();

	for i in 0..pipe_count {
		println!("pipe {}", i);
	
		let pipe = Pipe::new(
			width / 2, 0, 3
		);

		pipes.push(pipe);
	}

	// clear screen
	clear_screen();

    // loop
	let mut run = true;
	let mut cycles = 0;
	while run {
		//-- UPDATE
		// update pipes
		for pipe in &mut pipes {
			//- move pipe
			// convert position to an index
			let index_x: usize = pipe.x_pos.into();
			let index_y: usize = pipe.y_pos.into();
			
			// store last direction
			let last_direction = pipe.direction;

			// pick a new random direction
			if random() && random() && random() {
				// randomly turn left or right
				if random() {
					pipe.direction += 1;
				}
				else if pipe.direction == 0 {
					pipe.direction += 3;
				}
				else {
					pipe.direction -= 1;
				}

				// keep in bounds
				while pipe.direction > 3 {
					pipe.direction -= 4;
				}
			}

			// compare directions
			if pipe.direction != last_direction {
				// different direction, draw a +
				grid[index_x][index_y] = 3;
			}

			// move pipe forward
			if pipe.direction == 0 {		// left
				if pipe.x_pos > 0 {
					pipe.x_pos -= 1;
				}
				else {
					pipe.x_pos = width - 1;
				}
			}
			else if pipe.direction == 2 {	// right
				if pipe.x_pos < width - 1 {
					pipe.x_pos += 1;
				}
				else {
					pipe.x_pos = 0;
				}
			}
			else if pipe.direction == 1 {	// up
				if pipe.y_pos > 0 {
					pipe.y_pos -= 1;
				}
				else {
					pipe.y_pos = height - 4
				}
			}
			else if pipe.direction == 3 {	// down
				if pipe.y_pos < height - 4 {
					pipe.y_pos += 1;
				}
				else {
					pipe.y_pos = 0;
				}
			}
			
			//- mark pipe's new place
			// convert position to an index, again so that it draws in the new position
			let index_x: usize = pipe.x_pos.into();
			let index_y: usize = pipe.y_pos.into();
			
			// draw a | or -
			if pipe.direction == 0
			|| pipe.direction == 2 {
				grid[index_x][index_y] = 1;
			}
			else {
				grid[index_x][index_y] = 2;
			}
		}

		// cycles
		cycles += 1;
		if cycles > reset_cycles {
			run_pipes(
				pipe_count,
				update_speed,
				reset_cycles,
				show_cycles
			);
			run = false;
		}

		//-- DRAW
    	// header
    	if show_cycles {
    		print_at(format!("Pipes | Cycle: {}", cycles), 1, 0);
    	}
    	else {
    		print_at("Pipes".to_string(), 1, 0);
    	}

		for i in 0..width {
			print_at("_".to_string(), i, 1);
		}

		// draw grid
		for x in 0..width {
			// get a usize of x
			let index_x: usize = x.into();
	
			// loop y
			for y in 0..(height - 3) {
				// get a usize of y
				let index_y: usize = y.into();
			
				// get character
				let cell = match grid[index_x][index_y] {
					1 => "-".to_string(),
					2 => "|".to_string(),
					3 => "+".to_string(),
					_ => " ".to_string()
				};
	
				// print
				if &cell != " " {
					print_at(
						cell,
						x, y + 2
					);
				}
			}
		}

		// check if should exit
		if poll(Duration::from_millis(update_speed)).unwrap()
		&& read().unwrap() == Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty())) {
			run = false;
		}
	}

	// clear screen
	clear_screen();
}

// Print at a specific position
pub fn print_at(string: String, x_pos: u16, y_pos: u16) {
	if io::stdout().execute(MoveTo(x_pos, y_pos)).is_ok() {
		println!("{}", string);
	}
}

// Clear the screen
fn clear_screen() {
	if io::stdout().execute(Clear(ClearType::Purge)).is_ok() {
		// there has to be a better way to do this lmao
	}
}
