use kana2phone::kana2phone;

fn main() {
    // TODO: 標準入力を受け取って変換するところ

    let s = String::from("ホゲホゲホゲ");
    let s = kana2phone(&s);
    println!("{}", s);
}
