fn main() {
    let mut score = 100;
    score = 200;
    const BONUS:i32 = 30;
    //상수는 일반적으로 대문자로 표기, 타입 반듯이 적어야함.
    let result = score + BONUS;
    println!("result: {}", result);
}
