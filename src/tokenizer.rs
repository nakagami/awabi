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

pub struct Tokenizer {
    // system dictionary
    sys_dic: dic::MeCabDic,
    // user dictionary
    user_dic: Option<dic::MeCabDic>,

    // for unknown chars
    char_property: dic::CharProperty,
    unk_dic: dic::MeCabDic,

    // trans cost matrix
    matrix: dic::Matrix,
}

impl Tokenizer {
    pub fn new(mecabrc_path: Option<String>) -> Result<Tokenizer, std::io::Error> {
        let path = match mecabrc_path {
            Some(s) => s,
            None => "/etc/mecabrc".to_string(),
        };
        let rc = mecabrc::read(&path)?;
        let dicdir = &rc[&String::from("dicdir")];

        let sys_dic = dic::MeCabDic::open(&format!("{}/sys.dic", dicdir)).unwrap();
        let user_dic: Option<dic::MeCabDic> =
            if let Some(userdic_path) = rc.get(&String::from("userdic")) {
                Some(dic::MeCabDic::open(userdic_path).unwrap())
            } else {
                None
            };

        let char_property = dic::CharProperty::open(&format!("{}/char.bin", dicdir)).unwrap();
        let unk_dic = dic::MeCabDic::open(&format!("{}/unk.dic", dicdir)).unwrap();

        let matrix = dic::Matrix::open(&format!("{}/matrix.bin", dicdir)).unwrap();

        Ok(Tokenizer {
            sys_dic,
            user_dic,
            char_property,
            unk_dic,
            matrix,
        })
    }

    fn build_lattice(&self, s: &str) -> lattice::Lattice {
        let s = s.as_bytes();
        let mut lattice = lattice::Lattice::new(s.len());
        let mut pos = 0;
        while pos < s.len() {
            let mut matched: bool = false;

            // user_dic
            if let Some(user_dic) = &self.user_dic {
                let user_entries = user_dic.lookup(&s[pos..]);
                if user_entries.len() > 0 {
                    for entry in user_entries.into_iter() {
                        lattice.add(lattice::Node::new(entry), &self.matrix);
                    }
                    matched = true;
                }
            }

            // sys_dic
            let sys_entries = self.sys_dic.lookup(&s[pos..]);
            if sys_entries.len() > 0 {
                for entry in sys_entries.into_iter() {
                    lattice.add(lattice::Node::new(entry), &self.matrix);
                }
                matched = true;
            }

            // unknown
            let (unk_entries, invoke) =
                self.unk_dic.lookup_unknowns(&s[pos..], &self.char_property);
            if invoke || !matched {
                for entry in unk_entries.into_iter() {
                    lattice.add(lattice::Node::new(entry), &self.matrix);
                }
            }

            pos += lattice.forward();
        }
        lattice.end(&self.matrix);
        lattice
    }

    pub fn tokenize(&self, s: &str) -> Vec<(String, String)> {
        let lattice = self.build_lattice(s);
        let nodes = lattice.backward();
        let mut entries: Vec<(String, String)> = Vec::new();
        assert!(nodes[0].is_bos());
        assert!(nodes[nodes.len() - 1].is_eos());
        for i in 1..nodes.len() - 1 {
            let d = nodes[i].get_dic_entry();
            entries.push((d.original, d.feature));
        }
        entries
    }
}
