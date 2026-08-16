
fn main() {
    let x = 3;
    println!("x의 값은 {x}입니다.");

    let x = x + 1;
    println!("x의 값은 {x}입니다.");
    {
        let x = x * 2;
        println!("안쪽 범위에서 x의 값은 {x}입니다.");
    }
    println!("x의 값은 {x}입니다.");

    // 연산

    // 더하기
    let add = 3 + 8;
    let sub = 26.5 - 2.1;

    // 곱하기
    let mul = 7 * 20;

    // 나누기
    let quotient = 12.0 / 3.14;
    let truncated = 7 / 5;
    
    // ...

    // compound 타입
    let t: (i32, bool, f64) = (32, true, 1.41);
    
    // let (x, y, z) = t;
    let x = t.0;
    let y = t.1;
    let z= t.2;

    println!("x = {x}, y = {y}, z = {z}");

    // array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let threes = [3; 100];
    let last = threes[99];
    println!("{last}");

    let hellos = ["헬로"; 10];
    println!("{:?}", hellos);
    
    // 함수
    a_function(5, 6);

    // 원 넓이
    let a = circle_area(2.0);
    println!("원의 넓이는 {a}입니다.");


    // 조건문
    condition_statement();
    loop_statement();
}

// 함수
fn a_function(x : i32, y : i32) {
    let sum = x + y;
    println!("다른 함수입니다, {sum}"); 
}

const PI: f64 = 3.141592;
fn circle_area(radius: f64) -> f64 {
    let r2 = radius * radius;
    PI * r2
}

fn condition_statement() {
    // 조건문
    let x = 4;
    let condition = false;

    let y = if condition { 3 } else { 5 };
    println!("y는 {y}입니다.");

    if x % 3 == 0 {
        println!("x는 3으로 나누어 떨어집니다.");
    } else if x % 3 == 1 {
        println!("x는 3으로 나눈 나머지는 1입니다.");
    } else {
        println!("x는 3으로 나눈 나머지는 2이빈디ㅏ.");
    }

}

fn loop_statement() {
    // 무한 반복
    // loop {
    //     println!("반복");
    // }

    let mut counter = 0;
    loop {
        println!("{counter}");
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    let x = loop {
        println!("반복");
        counter += 1;
        if counter == 20 {
            break counter;
        }
    };

    println!("x = {x}");
    

    // while 문
    while (counter < 30) {
        println!("{counter}");
        counter += 1;
    }

    let arr = [1, 2, 3, 4, 5];
    let mut idx = 0;
    while (idx < arr.len()) {
        println!("arr[{}] = {}", idx, arr[idx]);
        idx += 1;
    }

    // for 문
    for x in arr {
        print!("x = {}, ", x);
    }
    println!();
    println!("완료");

    for i in (0..5) {
        println!("i = {i}");
    }

    let xs = ['가', '나', '다', '라', '마'];
    let l = xs.len();
    for i in (0..l) {
        println!("요소를 불러와라 = {}", xs[i]);
    }
    for i in (0..l).rev() {
        println!("거꾸로 = {}", xs[i]);
    }
}

