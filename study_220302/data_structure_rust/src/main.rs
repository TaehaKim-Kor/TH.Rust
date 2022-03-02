fn main() {
    let guess1: u32 = "42".parse().expect("Not an Integer!");
    let guess2: f32 = "42.0".parse().expect("Not a Float!");
    println!("guess1 is {}, guess2 is {}",guess1, guess2);
    // guess 변수를 uint32로 정하였다.
    // parse()는 문자열을 숫자로 리턴해주는 건가? 아무튼 숫자로 리턴해준다고 한다.
    // "42"라는 문자열을 guess에 parse로 숫자형으로 넣어준다.
    // 그러나 parse는 정수형(int)과 실수형(float)을 모두 리턴할 수 있기 때문에,
    // 만약 let guess = "42".parse().expect(msg:"Not a number!"); 라고 작성하면(자료형을 안 주면)
    // 컴파일이 안 된다. 컴파일러가 int인지 float인지 알 길이 없기 때문이다.
    let myarray: [u8;5] = [3;5];
    let mytuple = (1,2,3,4,5);
    println!("myarray's first element is {}, mytuple's second element is {}",myarray[0],mytuple.1);
    // Rust는 자료형(Data type)을 분명히해줘야한다. 그래도 이걸 컴파일러가 알려주긴하니까 짱임.

}
