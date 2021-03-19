use phf::phf_map;

static PHONE_DICT: phf::Map<&'static str, &str> = phf_map! {
    "ア" => "a",
    "イ" => "i",
    "ウ" => "u",
    "エ" => "e",
    "オ" => "o",
    "カ" => "k a",
    "キ" => "k i",
    "ク" => "k u",
    "ケ" => "k e",
    "コ" => "k o",
    "サ" => "s a",
    "ス" => "s u",
    "セ" => "s e",
    "ソ" => "s o",
    "タ" => "t a",
    "テ" => "t e",
    "ト" => "t o",
    "ナ" => "n a",
    "ニ" => "n i",
    "ヌ" => "n u",
    "ネ" => "n e",
    "ノ" => "n o",
    "ハ" => "h a",
    "ヒ" => "h i",
    "フ" => "f u",
    "ヘ" => "h e",
    "ホ" => "h o",
    "マ" => "m a",
    "ミ" => "m i",
    "ム" => "m u",
    "メ" => "m e",
    "モ" => "m o",
    "ヤ" => "y a",
    "ユ" => "y u",
    "ヨ" => "y o",
    "ラ" => "r a",
    "リ" => "r i",
    "ル" => "r u",
    "レ" => "r e",
    "ロ" => "r o",
    "ワ" => "w a",
    "ン" => "N",
    "ガ" => "g a",
    "ギ" => "g i",
    "グ" => "g u",
    "ゲ" => "g e",
    "ゴ" => "g o",
    "ザ" => "z a",
    "ジ" => "j i",
    "ズ" => "z u",
    "ゼ" => "z e",
    "ゾ" => "z o",
    "ダ" => "d a",
    "ヂ" => "j i",
    "ヅ" => "z u",
    "デ" => "d e",
    "ド" => "d o",
    "バ" => "b a",
    "ビ" => "b i",
    "ブ" => "b u",
    "ベ" => "b e",
    "ボ" => "b o",
    "パ" => "p a",
    "ピ" => "p i",
    "プ" => "p u",
    "ペ" => "p e",
    "ポ" => "p o",
    "ー" => ":",
    "ヲ" => "o",
    "トゥ" => "t u",
    "ジェ" => "j e",
    "ツァ" => "ts a",
    "ヴォ" => "b o",
    "ツィ" => "ts i",
    "キャ" => "ky a",
    "キュ" => "ky u",
    "キョ" => "ky o",
    "シャ" => "sh a",
    "シュ" => "sh u",
    "シェ" => "sh e",
    "ショ" => "sh o",
    "チャ" => "ch a",
    "チュ" => "ch u",
    "チェ" => "ch e",
    "チョ" => "ch o",
    "ツェ" => "ts e",
    "ニャ" => "ny a",
    "ニュ" => "ny u",
    "ニョ" => "ny o",
    "ヒャ" => "hy a",
    "ヒュ" => "hy u",
    "ヒョ" => "hy o",
    "ミャ" => "my a",
    "ミュ" => "my u",
    "ミョ" => "my o",
    "リャ" => "ry a",
    "リュ" => "ry u",
    "リョ" => "ry o",
    "ギャ" => "gy a",
    "ギュ" => "gy u",
    "ギョ" => "gy o",
    "ビャ" => "by a",
    "ビュ" => "by u",
    "ビョ" => "by o",
    "ヂュ" => "dy u",
    "ピャ" => "py a",
    "ピュ" => "py u",
    "ピョ" => "py o",
    "ティ" => "t i",
    "ファ" => "f a",
    "フィ" => "f i",
    "フェ" => "f e",
    "フォ" => "f o",
    "ジャ" => "j a",
    "ジュ" => "j u",
    "ジョ" => "j o",
    "ディ" => "d i",
    "デュ" => "d u",
    "ウェ" => "w e",
    "ウィ" => "w i",
    "ヴァ" => "b a",
    "ヴィ" => "b i",
    "ヴェ" => "b e",
    "ウォ" => "w o",
    "ズィ" => "j i",
    "ジァ" => "j a",
    "ドゥ" => "d u",
    "フョ" => "hy o",
    "フュ" => "hy u",
    "イェ" => "i e",
    "ツォ" => "ts o",
    "ニェ" => "n e",
    "ヒェ" => "h e",
    "ブィ" => "b i",
    "ミェ" => "m e",
    "クヮ" => "k a",
    "グヮ" => "g a",
    "スィ" => "sh i",
    "テュ" => "ts u",
    "ヴ" => "b u",
    "ツ" => "ts u",
    "シ" => "sh i",
    "チ" => "ch i",
    "ヮ" => "w a",
};

fn main() {
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
            None => panic!("この文字列は扱えないウホ: {}", s)
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
        assert_eq!(kana2phone(&String::from("カキクケコ")), "k a k i k u k e k o");
        assert_eq!(
            kana2phone(&String::from("パピプペポ")),
            "p a p i p u p e p o"
        );
    }

    #[test]
    fn test2(){
        assert_eq!(
            kana2phone(&String::from("チヮヮ")),
            "ch i w a w a"
        );

        assert_eq!(
            kana2phone(&String::from("ピャピュピョ")),
            "py a py u py o"
        );
    }
}
