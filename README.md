# awabi

`awabi` is a morphological analyzer using mecab dictionary.

## Requirements

MeCab
https://taku910.github.io/mecab/

For example, for debian/ubuntu

```
$ sudo apt install mecab
$ cargo install awabi
```

## How to use

```
$ echo 'すもももももももものうち'  | awabi
すもも  名詞,一般,*,*,*,*,すもも,スモモ,スモモ
も      助詞,係助詞,*,*,*,*,も,モ,モ
もも    名詞,一般,*,*,*,*,もも,モモ,モモ
も      助詞,係助詞,*,*,*,*,も,モ,モ
もも    名詞,一般,*,*,*,*,もも,モモ,モモ
の      助詞,連体化,*,*,*,*,の,ノ,ノ
うち    名詞,非自立,副詞可能,*,*,*,うち,ウチ,ウチ
```

