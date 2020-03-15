/*
*MIT License
*
*Copyright (c) 2020 Hajime Nakagami
*
*Permission is hereby granted, free of charge, to any person obtaining a copy
*of this software and associated documentation files (the "Software"), to deal
*in the Software without restriction, including without limitation the rights
*to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
*copies of the Software, and to permit persons to whom the Software is
*furnished to do so, subject to the following conditions:
*
*The above copyright notice and this permission notice shall be included in all
*copies or substantial portions of the Software.
*
*THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
*IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
*FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
*AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
*LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
*OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
*SOFTWARE.
*/
use super::*;

#[test]
fn test_tokenizer() {
    let tokenizer = tokenizer::Tokenizer::new(None).unwrap();

    assert_eq!(
        tokenizer.tokenize("すもももももももものうち"),
        vec![
            (
                "すもも".to_string(),
                "名詞,一般,*,*,*,*,すもも,スモモ,スモモ".to_string()
            ),
            ("も".to_string(), "助詞,係助詞,*,*,*,*,も,モ,モ".to_string()),
            (
                "もも".to_string(),
                "名詞,一般,*,*,*,*,もも,モモ,モモ".to_string()
            ),
            ("も".to_string(), "助詞,係助詞,*,*,*,*,も,モ,モ".to_string()),
            (
                "もも".to_string(),
                "名詞,一般,*,*,*,*,もも,モモ,モモ".to_string()
            ),
            ("の".to_string(), "助詞,連体化,*,*,*,*,の,ノ,ノ".to_string()),
            (
                "うち".to_string(),
                "名詞,非自立,副詞可能,*,*,*,うち,ウチ,ウチ".to_string()
            ),
        ]
    );

    assert_eq!(
        tokenizer.tokenize("祖父は１９０１年１０月２０日生まれです。"),
        vec![
            (
                "祖父".to_string(),
                "名詞,一般,*,*,*,*,祖父,ソフ,ソフ".to_string()
            ),
            ("は".to_string(), "助詞,係助詞,*,*,*,*,は,ハ,ワ".to_string()),
            ("１".to_string(), "名詞,数,*,*,*,*,１,イチ,イチ".to_string()),
            (
                "９".to_string(),
                "名詞,数,*,*,*,*,９,キュウ,キュー".to_string()
            ),
            ("０".to_string(), "名詞,数,*,*,*,*,０,ゼロ,ゼロ".to_string()),
            ("１".to_string(), "名詞,数,*,*,*,*,１,イチ,イチ".to_string()),
            (
                "年".to_string(),
                "名詞,接尾,助数詞,*,*,*,年,ネン,ネン".to_string()
            ),
            (
                "１０月".to_string(),
                "名詞,副詞可能,*,*,*,*,１０月,ジュウガツ,ジューガツ".to_string()
            ),
            ("２".to_string(), "名詞,数,*,*,*,*,２,ニ,ニ".to_string()),
            ("０".to_string(), "名詞,数,*,*,*,*,０,ゼロ,ゼロ".to_string()),
            (
                "日".to_string(),
                "名詞,接尾,助数詞,*,*,*,日,ニチ,ニチ".to_string()
            ),
            (
                "生まれ".to_string(),
                "名詞,一般,*,*,*,*,生まれ,ウマレ,ウマレ".to_string()
            ),
            (
                "です".to_string(),
                "助動詞,*,*,*,特殊・デス,基本形,です,デス,デス".to_string()
            ),
            ("。".to_string(), "記号,句点,*,*,*,*,。,。,。".to_string()),
        ]
    );
}
