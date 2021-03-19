use kana2phone::kana2phone;

fn main() {
    let s = String::from("ホゲホゲホゲ");
    let s = kana2phone(&s);
    dbg!(s);
}
