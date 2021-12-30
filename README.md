# awabi

`awabi` is a morphological analyzer using mecab dictionary, written in Rust.

## Requirements and how to install

MeCab https://taku910.github.io/mecab/ and related dictionary is required.

### Debian/Ubuntu
```
$ sudo apt install mecab mecab-ipadic-utf8
$ cargo install awabi
```

### Mac OS X (homebrew)
```
$ brew install mecab
$ brew install mecab-ipadic
$ cargo install awabi
```

## How to use

### use as library

#### Best tokens

example
```
use awabi::tokenizer;
fn main() {
    let tokenizer = tokenizer::Tokenizer::new(None).unwrap();
    let tokens = tokenizer.tokenize("すもももももももものうち");
    for token in tokens.iter() {
        println!("{}\t{}", token.0, token.1);
    }
}
```

result
```
すもも	名詞,一般,*,*,*,*,すもも,スモモ,スモモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
の	助詞,連体化,*,*,*,*,の,ノ,ノ
うち	名詞,非自立,副詞可能,*,*,*,うち,ウチ,ウチ
```

#### N-best tokens

example
```
use awabi::tokenizer;
fn main() {
    let tokenizer = tokenizer::Tokenizer::new(None).unwrap();
    let tokens_list = tokenizer.tokenize_n_best("すもももももももものうち", 3);
    for tokens in tokens_list.iter() {
        println!("------------------------------------------------------");
        for token in tokens.iter() {
            println!("{}\t{}", token.0, token.1);
        }
    }
}
```

result
```
------------------------------------------------------
すもも	名詞,一般,*,*,*,*,すもも,スモモ,スモモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
の	助詞,連体化,*,*,*,*,の,ノ,ノ
うち	名詞,非自立,副詞可能,*,*,*,うち,ウチ,ウチ
------------------------------------------------------
すもも	名詞,一般,*,*,*,*,すもも,スモモ,スモモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
の	助詞,連体化,*,*,*,*,の,ノ,ノ
うち	名詞,非自立,副詞可能,*,*,*,うち,ウチ,ウチ
------------------------------------------------------
すもも	名詞,一般,*,*,*,*,すもも,スモモ,スモモ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
の	助詞,連体化,*,*,*,*,の,ノ,ノ
うち	名詞,非自立,副詞可能,*,*,*,うち,ウチ,ウチ
```

#### Use the specified mecabrc

```
use awabi::tokenizer;
fn main() {
    let tokenizer = tokenizer::Tokenizer::new(Some("/some/where/mecabrc")).unwrap();
    let tokens = tokenizer.tokenize("すもももももももものうち");
    for token in tokens.iter() {
        println!("{}\t{}", token.0, token.1);
    }
}
```

### awabi command

```
$ echo 'すもももももももものうち'  | awabi
すもも  名詞,一般,*,*,*,*,すもも,スモモ,スモモ
も      助詞,係助詞,*,*,*,*,も,モ,モ
もも    名詞,一般,*,*,*,*,もも,モモ,モモ
も      助詞,係助詞,*,*,*,*,も,モ,モ
もも    名詞,一般,*,*,*,*,もも,モモ,モモ
の      助詞,連体化,*,*,*,*,の,ノ,ノ
うち    名詞,非自立,副詞可能,*,*,*,うち,ウチ,ウチ
EOS
```
```
$ echo 'すもももももももものうち'  | awabi -N 2
すもも  名詞,一般,*,*,*,*,すもも,スモモ,スモモ
も      助詞,係助詞,*,*,*,*,も,モ,モ
もも    名詞,一般,*,*,*,*,もも,モモ,モモ
も      助詞,係助詞,*,*,*,*,も,モ,モ
もも    名詞,一般,*,*,*,*,もも,モモ,モモ
の      助詞,連体化,*,*,*,*,の,ノ,ノ
うち    名詞,非自立,副詞可能,*,*,*,うち,ウチ,ウチ
EOS
すもも  名詞,一般,*,*,*,*,すもも,スモモ,スモモ
も      助詞,係助詞,*,*,*,*,も,モ,モ
もも    名詞,一般,*,*,*,*,もも,モモ,モモ
もも    名詞,一般,*,*,*,*,もも,モモ,モモ
も      助詞,係助詞,*,*,*,*,も,モ,モ
の      助詞,連体化,*,*,*,*,の,ノ,ノ
うち    名詞,非自立,副詞可能,*,*,*,うち,ウチ,ウチ
EOS
```

## See also

- pyawabi https://github.com/nakagami/pyawabi Python wrapper
- exawabi https://github.com/nakagami/exawabi Elixir wrapper
- goawabi https://github.com/nakagami/goawabi Go implementation
- pure-pyawabi https://github.com/nakagami/pure-pyawabi Python implementation
- Awabi.jl https://github.com/nakagami/Awabi.jl Julia implementation
