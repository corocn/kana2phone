mod dictionary;
use dictionary::{PhoneDict, PHONE_DICT_1, PHONE_DICT_2};

fn main() {
    let s = String::from("ツトゥトゥーイェアー");
    let s = kana2phone(&s);
    dbg!(s);
}

fn replace_to_phoneme(text: &str, dict: &PhoneDict) -> String {
    let mut s: String = text.to_string();

    let keys = dict.keys();
    for key in keys {
        let replace_text = format!(" {} ", dict[*key]);
        s = s.replace(*key, &replace_text);
    }

    s
}

fn kana2phone(text: &str) -> String {
    let s = String::from("ツトゥトゥーイェアー");
    let s = replace_to_phoneme(&s, &PHONE_DICT_2);
    let s = replace_to_phoneme(&s, &PHONE_DICT_1);
    let s = s.replace("  ", " ");
    let s = s.trim().to_string();

    s
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
