fn main() {
    println!("Hello, world!");

    {
        let s: &str = "hello";
        // s が hello を所有している（変更不可能）
        println!("{}", s);
    }
    // println!("{}", s);

    // String 型はヒープに置かれる（変更可能）
    // 厳密には、ヒープ領域へのポインタ
    {
        let mut s: String = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
    }
    // ブロックを抜けるとき、使ったヒープ領域をOSに返す

    // ムーブ
    let s1 = String::from("hello");
    let s2 = s1; // 👀
    println!("{}", s2);
    // s1 と s2 は同じヒープ領域を指していない❌
    // s2 = s1 の時点で、ヒープ領域がコピーされる? ❌
    // s2 = s1 の時点で、s1は使えなくなっている！

    // コピーするなら clone をつける
    let s3 = s2.clone();
    println!("{}", s3);

    /*
    所有権の移動・・・
    ヒープ領域を使う変数は、
    それをどこかに代入した（＝他の変数に入れた、関数の引数にした）
    とき、その変数の所有権は移動する。-> 元の変数は使えない
    */

    // 変数を関数の引数に入れて、その関数に所有権が移動したとする。
    // 関数から返された所有権を、元の変数に束縛し直す
    let t1 = String::from("hello");
    let t2 = return_string(t1);
    println!("{}", t2);

    // 所有権の借用
    // これは不変参照
    let t3 = String::from("hello");
    let len = calc_len(&t3);

    // 可変参照
    // 1つのデータに対し、可変参照は1つだけ（∵データの競合）
    let mut t4 = String::from("hello");
    add_world(&mut t4);
    println!("{}", t4);

}

// 引数の所有権は、関数が持つ
// その所有権を返す
fn return_string(s: String) -> String {
    println!("return ownership: {}", s);
    s
}

// 不変参照
fn calc_len(s: &String) -> usize {
    s.len()
}

// 可変参照
fn add_world(s: &mut String) {
    s.push_str(", world!");
}
