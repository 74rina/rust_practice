fn main() {
    println!("Hello, world!");

    // 変数宣言
    // 再代入（mut指定）
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const word: usize = 100;

    // 再宣言（シャドーイング）
    let y: i32 = 5;
    let y: i32 = y + 1;
    {
        let y: i32 = y * 2;
        println!("The value of x is: {}. from block", y);
    }
    println!("The value of x is: {}", y);
    
    let strings: &str = "aaa";
    let strings: usize = strings.len();
        // usize を使うことが多い
    println!("The value of strings is: {}", strings);


    // データ型
    let x: usize = 6;
    let y: f64 = 1.5;
    // let z = x / y; -> error!
    let z: f64 = (x as f64) / y;
    println!("z: {}", z);
    
    let x: (u32, f64, usize) = (500, 6.4, 1); // tuple
    // let a = [1, 2, 3];
    // let a: [i32, 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];

    println!("my sum function: {}", add(2, 3));

    // 条件分岐
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("The value of num is: {}", num);

    // 無限ループ
    let mut count = 0;
    loop {
        if count == 3 {
            break;
        }
        println!("Hello");
        count += 1
    }

    // 配列のループ
    for elm in 0..5 {
        println!("{}", elm);
    }

    // 構造体のインスタンスを作成・メソッドにアクセス
    let new_user = User{ name: String::from("rina"), age: 20 };
    new_user.greet();

    // enum型の使用例
    let light = TrafficLight::Red;
    action(light);

}

// 関数
// 末尾に式（セミコロンなし）を書くと、それが返り値になる
// セミコロンをつけると、式ではなく文となる
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// 構造体
struct User {
    name: String,
    age: u32,
}
impl User {
    fn greet(&self) {
        println!("Hello, {}!", &self.name);
    }
}

// enum型（複数の値を取りうる型）
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

enum TrafficLight {
    Red,
    Yellow,
    Green
}
fn action(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!{"止まる"},
        TrafficLight::Yellow => println!{"注意する"},
        TrafficLight::Green => println!{"進む"},
    }
}
