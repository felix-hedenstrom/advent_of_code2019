use std::io::{self, Read};

fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer.clone()
}


fn calculate_required_fuel(mass: &i64) -> i64{

	return (mass / 3) - 2;

}

fn calculate_fuel_for_fuelmass(fuelmass: &i64) -> i64{
	let required_fuel = calculate_required_fuel(fuelmass); 
	if required_fuel <= 0 {
		return 0;
	}
	return required_fuel + calculate_fuel_for_fuelmass(&required_fuel);
}

fn main (){
	let input: Vec<i64> =
		read_stdin()
		.trim()
		.split("\n")
		.map(|s| 
			s.parse::<i64>().expect("of of the lines of the input could not be parsed into an integer") 
		).collect();
	

	let answer: i64 =
		input
		.iter()
		.map(calculate_required_fuel)
		.sum();	

	let answer_part2: i64 =
		input
		.iter()
		.map(calculate_fuel_for_fuelmass)
		.sum();	

	print!("{:?}\n", input);
	print!("{:?}\n", answer);
	print!("{:?}\n", answer_part2);
}
