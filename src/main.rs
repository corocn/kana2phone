mod dictionary;
use dictionary::{PhoneDict, PHONE_DICT_1, PHONE_DICT_2};
use regex::Regex;

fn main() {
    let s = String::from("ツトゥトゥーーーイェアー");
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
    let s = text.to_string();
    let s = replace_to_phoneme(&s, &PHONE_DICT_2);
    let s = replace_to_phoneme(&s, &PHONE_DICT_1);
    let s = s.replace("  ", " ");
    let s = s.trim().to_string();
    let s = s.replace("  ", " ");
    let s = s.replace(" :", ":");

    let re = Regex::new(r":+").unwrap();
    let s = re.replace_all(&s, ":").to_string();

    let re2 = Regex::new(r"^:\s").unwrap();
    let s = re2.replace_all(&s, "").to_string();

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

    #[test]
    fn test3() {
        assert_eq!(kana2phone(&String::from("ダンボール")), "d a N b o: r u");
        assert_eq!(
            kana2phone(&String::from("ダンボーーール")),
            "d a N b o: r u"
        );
        assert_eq!(
            kana2phone(&String::from("ーーダンボーーールーー")),
            "d a N b o: r u:"
        );
    }
}
