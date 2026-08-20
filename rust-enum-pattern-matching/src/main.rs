// Enum
// - 여러 정해진 가짓수 중 택일 값을 다루는 타입
// - 가위 | 바위 | 보 등

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

enum Message {
    StartGame,
    WinPoint { who: String },
    ChangePlayerName(String),
}

// 이미 있음
// enum Option<T> {
//     None,
//     Some(T),
// }

struct RGB(u8, u8, u8);

// 패턴 매칭
fn color_to_rgb(color: Color) -> RGB {
    // match는 모든 경우에 대해 처리해야 함
    match color {
        Color::Red => RGB(255, 0, 0),
        Color::Green => RGB(0, 255, 0),
        Color::Blue => RGB(0, 0, 255),
    }
}

fn handle_message(message: Message) {
    match message {
        Message::StartGame => println!("게임시작!"),
        Message::WinPoint { who } => println!("{}의 득점", who),
        // Message::ChangePlayerName(name) => println!("플레이어 이름 변경 => {}", name),

        // ChangePlayerName이 처리되지 않았을 때, 와일드카드로 처리 가능
        // _ => println!("아직 구현하지 않은 메시지");

        // 혹은 뭘 받든 상관없을 때
        Message::ChangePlayerName(_) => println!("플레이어 이름 변경됨"), // 이렇게 받는 인자를 와일드 카드로 처리 가능
    }
}

fn increment(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let red: Color = Color::Red; // 타입은 Color라는 enum
    let green = Color::Green;

    println!("red = {:?}", red);
    println!("red == green => {}", red == green);
    println!("red == red => {}", red == Color::Red);

    let m1 = Message::StartGame;
    let m2 = Message::WinPoint {
        who: String::from("홍길동"),
    };
    let m3 = Message::ChangePlayerName(String::from("둘리"));

    let some_number = Some(2);
    let absent_number: Option<i32> = None;

    // some_number + 1; // 오류 - 둘이 타입이 달라서 연산 불가능

    // enum 안의 값 연산
    let x = Some(2);
    println!("{:?}", increment(x)); // => Some(3)
    println!("{:?}", increment(None)); // => None
}
