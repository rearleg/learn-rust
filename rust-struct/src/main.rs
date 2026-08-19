#[derive(Clone)]
struct User {
    name: String,
    email: String,
    active: bool,
}

// 튜플 구조체
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(name: String, email: String) -> User {
    User {
        // name: name,
        // email: email,
        // active: true,
        // 생략도 가능하다.
        name,
        email,
        active: true,
    }
}

fn main() {
    let mut user = build_user(String::from("홍길동"), String::from("hong@gmail.com"));

    user.email = String::from("gd.hong@gmail.com");
    println!("이용자의 이름은 = {}", user.name);
    println!("이용자의 이메일은 = {}", user.email);

    // 이미 있는 구조체 인스턴스로부터 새로 만들기
    let user1 = User {
        name: String::from("이순신"),
        email: String::from("lee@gmail.com"),
        active: true,
    };

    let user2 = User {
        name: user1.name,
        email: user1.email,
        active: false,
    };

    let user3 = User {
        active: false,
        ..user2.clone() // 이렇게 축약도 가능
    };

    println!("user2.email = {}", user2.email);

    // println!("user1.email = {}", user1.email); // 이건 소유권이 넘어갔기 때문에 안됨
    // .clone()으로 해결!




    // 튜플 구조체
    // 튜플 구조체는 튜플과 다르게 이름을 붙일 수 있다.
    let color = Color(1, 2, 3);
    let point = Point(1, 2, 3);
    color.0;
    color.1;


    // 사격형 너비와 높이로 면적 구하기
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    println!("해당 사격형의 면적은 {}.", rect.area());
    // println!("사각형 = {:?}", rect);
    // dbg!(rect);  // 이것도 가능
    println!("정사각형 = {:?}", Rectangle::square(20))
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 구조체 요약
// - 관련있는 값들을 이름도 붙여 모아 구성하는 타입
// - impl 블록으로 메서드나 연관함수 만들어 사용
// - 메소드의 첫번째 파라미터는 self, &self, &mut self 모두 가능
