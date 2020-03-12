/*
* Copyright (c) 2020, Hajime Nakagami
* All rights reserved.
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice, this
*    list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
*    this list of conditions and the following disclaimer in the documentation
*    and/or other materials provided with the distribution.
*
* 3. Neither the name of the copyright holder nor the names of its
*    contributors may be used to endorse or promote products derived from
*    this software without specific prior written permission.
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
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
