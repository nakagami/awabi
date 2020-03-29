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
        let path = if let Some(s) = mecabrc_path {
            s
        } else {
            mecabrc::find_mecabrc().expect("Can't find mecabrc")
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

    pub fn tokenize_n_best(&self, s: &str, n: u32) -> Vec<Vec<(String, String)>> {
        let lattice = self.build_lattice(s);
        let nodes_vec = lattice.backward_astar(n, &self.matrix);
        let mut entries_vec: Vec<Vec<(String, String)>> = Vec::new();
        for nodes in nodes_vec.iter() {
            // TODO: convert nodes_vec to entries_vec
            let mut entries: Vec<(String, String)> = Vec::new();
            for i in 1..nodes.len() - 1 {
                let d = nodes[i].get_dic_entry();
                entries.push((d.original, d.feature));
            }
            entries_vec.push(entries);
        }

        entries_vec
    }
}
