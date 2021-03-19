use phf::phf_map;

mod dictionary;
use dictionary::PHONE_DICT;

fn main() {
    println!("{}", AIUEO);

    let text = String::from("アイウエオ");
    let phone = kana2phone(&text);

    println!("{}", phone);
}

fn kana2phone(text: &str) -> String {
    let v: Vec<&str> = text.split("").collect();
    let v: Vec<&str> = v
        .iter()
        .filter(|&&x| x != String::from(""))
        .cloned()
        .collect::<Vec<&str>>();

    let mut result: Vec<&str> = vec![];
    // let target = "";
    for s in v.iter().rev().cloned() {
        let p = match PHONE_DICT.get(s).cloned() {
            Some(s) => s,
            None => panic!("この文字列は扱えないウホ: {}", s),
        };
        result.push(p);
    }

    let rev: Vec<&str> = result.iter().rev().cloned().collect();
    rev.join(" ")
}

#[cfg(test)]
mod tests {
    use crate::kana2phone;

    #[test]
    fn test() {
        assert_eq!(kana2phone(&String::from("アイウエオ")), "a i u e o");
        assert_eq!(
            kana2phone(&String::from("カキクケコ")),
            "k a k i k u k e k o"
        );
        assert_eq!(
            kana2phone(&String::from("パピプペポ")),
            "p a p i p u p e p o"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(kana2phone(&String::from("チヮヮ")), "ch i w a w a");

        assert_eq!(kana2phone(&String::from("ピャピュピョ")), "py a py u py o");
    }
}
