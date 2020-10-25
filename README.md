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

### use as library

See main as sample code.

- tokensize https://github.com/nakagami/awabi/blob/master/src/main.rs#L58
- N best match https://github.com/nakagami/awabi/blob/master/src/main.rs#L60

## See also

- goawabi https://github.com/nakagami/goawabi Go implementation
- pyawabi https://github.com/nakagami/pyawabi Python wrapper
- pure-pyawabi https://github.com/nakagami/pure-pyawabi
- exawabi https://github.com/nakagami/exawabi
