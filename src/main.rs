use kana2phone::kana2phone;

fn main() {
    let s = readline();
    let s = kana2phone(&s);
    println!("{}", s);
}

fn readline() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned()
}
