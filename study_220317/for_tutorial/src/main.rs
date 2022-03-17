fn main() {
	for number in (1..4){ //== for(int i = 1; i<4; i++) / for number in range(1,4,1):
	//특이한건, 1..4만 써도 된다는게 신기.
    //근데 괄호로 묶어주면 왜 필요없다면서 주의표시를 띄워줄까? 이게 더 가독성이 좋다고 보는걸까?
		println!("{}",number);
	}
println!("END!");
}