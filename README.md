# kana2phone
a kana string to phoneme

## Usage

```
let s = String::from("コンニチハー");
let s = kana2phone(&s);
println!("{}", s);
```

```
$ echo "コンニチハー" | kana2phone
k o N n i ch i h a:
```
