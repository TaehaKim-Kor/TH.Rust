use std::mem; //메모리 관련 입출력 라이브러리 호출?
//함수의 기본 형태는 fn 함수이름(입력변수){소스코드}
fn new_function(x:i32) -> i32 { //리턴은 이렇게 정해줘야함.
    //x 를 parameter, argument라고 함.
    println!("Input value is {}", x);
    return x;
}
fn another_function(x:i32) -> i32{
    println!("Input value is {}",x);
    //그러나 Rust에서 세미콜론의 의미가 C와는 조금 다르다.
    x+1//이렇게만 적어놓으면 이 코드는 y+1을 리턴한단 의미이다.
    //세미콜론이 없으면 리턴하고 있으면 리턴하지 않음.
    //이걸 반드시 정해줘야함. Matlab과 유사한 느낌이 있음.
}

//참고 : Rust는 함수 오버로딩(overloading)을 지원하지 않는다!
fn new_function2(_owner_name:&str){
    println!("{} wants to know the function size_of's output of i64, and it is {}.", _owner_name, mem::size_of::<*const i64>());
}

fn main() {
    let x: i32 = 123;
    let myname: &str = "김태하";
    let y = new_function(x);
    let z = another_function(x);
    // let x = (let y = 3);
    //-> 이게 에러가 나오는 이유는, let x는 등호 이후의 값을 필요로 하는데,
    //let y = 3은 리턴값이 없기 때문임.

    println!("Returned value of new function is {}",y);
    println!("Returned value of another function is {}",z);
    new_function2(myname);
}
