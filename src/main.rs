// 世界最古のアルゴリズムであるユークリッドの互除法を使って
// 最大公約数を計算するプログラムを Rust で実装してください。
// 仕様
// ユーザが入力した任意の整数と、0 ~ 1000 までのランダムな整数の最大公約数を出力する
use std::io;
fn main() {
    println!("Please enter any two numbers.");
    let mut str_num1 = String::new();
    io::stdin()
        .read_line(&mut str_num1)
        .expect("Failed to read line");
    let mut num1: u32 = str_num1.trim().parse()
        .expect("Please type a number!");

    let mut str_num2 = String::new();
    io::stdin()
        .read_line(&mut str_num2)
        .expect("Failed to read line");
    let mut num2: u32 = str_num2.trim().parse()
        .expect("Please type a number!");
    loop {
        let remain = num1 % num2;
        if remain == 0 {
            println!("{}", num2);
            break;
        } else {
            num1 = num2; num2 = remain;
        }
    }
}