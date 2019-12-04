use std::io::{self, Read};

use std::convert::TryInto;

fn read_stdin() -> String{
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("did not recieve anything from stdin");
	return buffer.clone()
}

//#[derive(Debug, Clone)]
#[derive(Debug, Clone)]
struct Digits{
	digits: Vec<i8>
}

impl Digits{
	fn new(n: u64) -> Digits{
		let mut d: Vec<i8> = vec![];
		let mut mn: u64 = n;
		for _i in 0..n.to_string().len(){
			d.insert(0, ((mn % 10) as i8).try_into().unwrap());
			mn = mn / 10;
		}
		return Digits{ digits: d};
	}
	fn len(&self) -> usize{
		return self.digits.len();
	}
	fn get(&self, i: &usize) -> i8{
		return self.digits[*i];
	}
}

fn atleast_two_adjacent(n: &Digits) -> bool{
	for i in 0..(n.len() - 1){
		if n.get(&i) == n.get(&(i + 1)){
			return true;
		}
	}	
	return false;
}

fn only_two_adjacent(n: &Digits) -> bool{
	if n.get(&0) == n.get(&1) && n.get(&0) != n.get(&2){
		return true;	
	}
	let l = n.len() - 1;
	if n.get(&l) == n.get(&(l - 1) ) && n.get(&l) != n.get(&(l - 2)){
		return true;	
	}
	for i in 1..(n.len() - 2){
		//print!("{:?}, {:?}, {:?}, {:?}\n", n.get(&(i - 1)), n.get(&i), n.get(&(i + 1)), n.get(&(i + 2)));
		if n.get(&i) == n.get(&(i + 1)) && n.get(&i) != n.get(&(i - 1)) && n.get(&i) != n.get(&(i + 2)){
			return true;
		}
	}
	return false;
	
}

fn is_ascending(n: &Digits) -> bool{
	
	for i in 0..(n.len() - 1){
		//print!("{:?} <= {:?}\n", n.get(&i), n.get(&(i + 1)));
		if !(n.get(&i) <= n.get(&(i + 1))){
			return false;
		}
	}	
	return true;
}

fn passes_tests(n: u64) -> bool{
	let d = Digits::new(n);
	return atleast_two_adjacent(&d) && is_ascending(&d);
}
fn passes_tests_part2(n: u64) -> bool{

	let d = Digits::new(n);
	return is_ascending(&d) && only_two_adjacent(&d);
}

fn main (){

	let mut count: usize = 0;	
	/*
	print!("Trying input1: {:?}, {:?}\n", Digits::new(111111), passes_tests_part2(111111));
	print!("Trying input1: {:?}, {:?}\n", Digits::new(223450), passes_tests_part2(223450));
	print!("Trying input1: {:?}, {:?}\n", Digits::new(123789), passes_tests_part2(123789));
	print!("trying input1: {:?}, {:?}\n", Digits::new(112222), passes_tests_part2(112222));
	print!("trying input1: {:?}, {:?}\n", Digits::new(111122), passes_tests_part2(111122));
	print!("Trying input1: {:?}, {:?}\n", Digits::new(11122333), passes_tests_part2(11122333));
	print!("trying input1: {:?}, {:?}\n", Digits::new(112233), passes_tests_part2(112233));
	print!("trying input1: {:?}, {:?}\n", Digits::new(123444), passes_tests_part2(123444));
	*/
	for i in 231832..767346{
		if passes_tests_part2(i){
			count += 1
		}
	}
	
	print!("{:?}\n", count);
}
