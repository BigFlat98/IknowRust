fn main() {
    // [미션] 아래의 단계별 설명을 따라 코드를 작성해 보세요!
    // 각 단계마다 let 키워드를 다시 사용하여 'data' 변수를 Shadowing 하는 것이 핵심입니다.

    // 1단계: "123" 이라는 문자열을 담은 'data' 변수를 선언하세요.
    // 여기에 코드 작성:
    let data = "123";

    

    // 2단계: Shadowing을 사용하여 'data'를 정수(i32) 타입으로 변환하세요.
    // 힌트: let data: i32 = data.parse().expect("숫자 변환 실패!");
    // 여기에 코드 작성:
    let data:i32 = data.parse().expect("숫자 변환 실패!"); //parse는 문자열을 정수로 형변환

    // 3단계: Shadowing을 사용하여 기존 'data' 값에 77을 더한 값을 다시 'data'에 담으세요.
    // 여기에 코드 작성:
    let data = data + 77;



    // 4단계: 최종 'data' 값을 출력하여 200이 나오는지 확인하세요!
    // println!("최종 결과: {}", data);
    println!("결과: {}", data);



    //daily_problems_02
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);
}
