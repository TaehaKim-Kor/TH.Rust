fn main() {
	let number:[i8;5]  = [10,20,30,40,50];
	let mut index = 5;
  while index >= 1 {
		println!("{}",number[index-1]);
		index -= 1;
	}
	println!("END!")
}