use std::env;
use std::fs::File;
use std::io::{BufReader,BufRead};

struct Tableau {
	m: u16,
	n: u16,
	matrix: Vec<Vec<f64>>,
}

fn remove_unwanted_chars(text: &mut String) {
	text.retain(|c| c != 'm');
	text.retain(|c| c != 'i');
	text.retain(|c| c != 'n');
	text.retain(|c| c != ';');
	text.retain(|c| c != ':');
}

fn parse_file(problem: &mut Tableau, filename: &String) {
	let f = File::open(filename).unwrap();
	let mut row_counter: u64 = 0;

	// Read lines from file and convert them to string
	for line in BufReader::new(f).lines() {
		let mut line = String::from(line.unwrap());
		if &line[..2] != "//".to_string() {
			remove_unwanted_chars(&mut line);
			// Add row to tableau
			problem.matrix.push(Vec::new());
			problem.m += 1;
			// Split values, convert and enter into row
			line = line.replace(" >= ", " + ");
			let v: Vec<&str> = line.split(" + ").collect();
			for mut element in v {
				element.to_string();
				if !element.is_empty() {
					let index = element.find('*')
						.unwrap_or(element.len());
					element = &element[..index];
					problem.matrix[row_counter as usize].push(
						element.parse::<f64>().unwrap());
					problem.n += 1;
				}
			}
			row_counter += 1;
		}
	}
	problem.matrix[0].push(1.0);
	problem.n += 1;
	let tmp: Vec<f64> = problem.matrix.remove(0);
	problem.matrix.push(tmp);
	problem.n = problem.n / problem.m;
	//print_tableau(problem);
}

fn print_tableau(problem: &Tableau) {
	println!("{}", problem.m);
	println!("{}", problem.n);
	for row in &problem.matrix {
		for number in row {
			print!("{}", number.to_string());
			print!(",");
		}
		println!("");
	}
}

fn transpose (problem: &mut Tableau) -> Tableau {
	let mut result = Tableau {
		m: 0,
		n: 0,
		matrix: Vec::new(),
	};
	result.m = problem.n;
	result.n = problem.m;
	
	for _i in 0..problem.n {
		result.matrix.push(Vec::new());
	}
	
	for i in 0..problem.n {
		for j in 0..problem.m {
			result.matrix[i as usize].push(problem.matrix[j as usize][i as usize]);
		}
	}
	result
}

fn negate (problem: &mut Tableau) {
	for i in 0..problem.n - 1 {
		problem.matrix[(problem.m -1) as usize][i as usize] *= -1.0;
	}
}

fn add_slack_variables(problem: &mut Tableau) {
	for i in 0..problem.m {
		for j in 0.. problem.m {
			if i == j {
				problem.matrix[i as usize].insert((problem.n - 1 + j) as usize, 1.0);
			}
			else {
				problem.matrix[i as usize].insert((problem.n - 1 + j) as usize, 0.0);
			}
		}
	}
	problem.n += problem.m;
	problem.matrix[(problem.m - 1) as usize][(problem.n - 1) as usize] = 0.0;
}

fn check_for_negatives(problem: &Tableau) -> bool {
	let mut result = false;
	for element in &problem.matrix[(problem.m - 1) as usize] {
		if element < &0.0 {
			result = true;
		}
	}
	result
}

fn find_pivot_col(problem: &Tableau) -> u16 {
	let mut result: u16 = 0;
	let mut pivot: f64 = 0.0;
	for i in 0..problem.n {
		if problem.matrix[(problem.m - 1) as usize][i as usize] < pivot {
			pivot = problem.matrix[(problem.m - 1) as usize][i as usize];
			result = i;
		}
	}
	result
}

fn find_pivot_row(problem: &Tableau, pivot_col: &u16) -> u16 {
	let mut result: u16 = 0;
	let mut min_ratio: f64 = -1.0;
	for i in 0..problem.m {
		let ratio: f64 = problem.matrix[i as usize][(problem.n - 1) as usize] / problem.matrix[i as usize][*pivot_col as usize];
		if ((ratio > 0.0) & (ratio < min_ratio)) | (min_ratio < 0.0) {
			min_ratio = ratio;
			result = i;
		}
	}
	result
}

fn pivot_on(problem: &mut Tableau, pivot_col: &u16, pivot_row: &u16) {
	let pivot: f64 = problem.matrix[*pivot_row as usize][*pivot_col as usize];
	for i in 0..problem.n {
		problem.matrix[*pivot_row as usize][i as usize] /= pivot;
	}
	for i in 0..problem.m {
		let multiplier: f64 = problem.matrix[i as usize][*pivot_col as usize];
		if i != *pivot_row {
			for j in 0..problem.n {
				problem.matrix[i as usize][j as usize] -= multiplier * problem.matrix[*pivot_row as usize][j as usize];
			}
		}
	}
}

fn print_results(problem: &Tableau) {
	println!("Mit den Variablen:");
	for i in 0..(problem.m -1) {
		println!("x{} = {}", i, problem.matrix[i as usize][(problem.n - 1) as usize]);
	}
	println!("ist der minimale Wert {}", problem.matrix[(problem.m - 1) as usize][(problem.n - 1) as usize]);
}

fn main() {
	// Get filename from console argument
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	//let filename = "KI_20.txt".to_string();

	//Create tableau
	let mut problem = Tableau {
		m: 0,
		n: 0,
		matrix: Vec::new(),
	};

	// Parse file
	parse_file(&mut problem, &filename);
	// Transpose matrix
	problem = transpose(&mut problem);
	//print_tableau(&problem);
	// Negate last line
	negate(&mut problem);
	//print_tableau(&problem);
	// Add slack Variables
	add_slack_variables(&mut problem);
	//print_tableau(&problem);
	// Check for negative Values
	//println!("{}", check_for_negatives(&problem));
	// Start simplex loop
	//let mut looper: u16 = 0;
	while(check_for_negatives(&problem)) {
		//looper += 1;
		let pivot_col: u16 = find_pivot_col(&problem);
		let pivot_row: u16 = find_pivot_row(&problem, &pivot_col);
		// Start pivoting
		pivot_on(&mut problem, &pivot_col, &pivot_row);
		//print_tableau(&problem);
	}
	print_results(&problem);
	//println!("Completed!");
}