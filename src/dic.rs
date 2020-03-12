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

extern crate memmap;

use memmap::{Mmap, MmapOptions};
use std::fs::File;
use std::i16;
use std::i32;
use std::str;
use std::u16;
use std::u32;

fn unpack_u32(mmap: &Mmap, i: usize) -> u32 {
    u32::from_le_bytes([mmap[i], mmap[i + 1], mmap[i + 2], mmap[i + 3]])
}

fn unpack_i32(mmap: &Mmap, i: usize) -> i32 {
    i32::from_le_bytes([mmap[i], mmap[i + 1], mmap[i + 2], mmap[i + 3]])
}

fn unpack_u16(mmap: &Mmap, i: usize) -> u16 {
    u16::from_le_bytes([mmap[i], mmap[i + 1]])
}

fn unpack_i16(mmap: &Mmap, i: usize) -> i16 {
    i16::from_le_bytes([mmap[i], mmap[i + 1]])
}

fn unpack_string(mmap: &Mmap, offset: usize) -> String {
    let mut end = offset;
    while mmap[end] != 0 {
        end += 1;
    }
    str::from_utf8(&mmap[offset..end]).unwrap().to_string()
}

fn utf8_to_ucs2(s: &[u8], index: usize) -> (u16, usize) {
    // utf8 to ucs2(16bit) code and it's array size
    let ln = if (s[index] & 0b10000000) == 0b00000000 {
        1
    } else if (s[index] & 0b11100000) == 0b11000000 {
        2
    } else if (s[index] & 0b11110000) == 0b11100000 {
        3
    } else if (s[index] & 0b11111000) == 0b11110000 {
        4
    } else {
        0
    };

    let mut ch32: u32 = 0;
    match ln {
        1 => ch32 = s[index + 0] as u32,
        2 => {
            ch32 = ((s[index + 0] & 0x1F) as u32) << 6;
            ch32 |= (s[index + 1] & 0x3F) as u32;
        }
        3 => {
            ch32 = ((s[index + 0] & 0x0F) as u32) << 12;
            ch32 |= ((s[index + 1] & 0x3F) as u32) << 6;
            ch32 |= (s[index + 2] & 0x3F) as u32;
        }
        4 => {
            ch32 = ((s[index + 0] & 0x07) as u32) << 18;
            ch32 |= ((s[index + 1] & 0x3F) as u32) << 12;
            ch32 |= ((s[index + 2] & 0x3F) as u32) << 6;
            ch32 |= (s[index + 3] & 0x03F) as u32;
        }
        _ => ch32 = 0,
    }

    // ucs4 to ucs2
    let ch16 = if ch32 < 0x10000 {
        ch32 as u16
    } else {
        ((((ch32 - 0x10000) / 0x400 + 0xD800) << 8) + ((ch32 - 0x10000) % 0x400 + 0xDC00)) as u16
    };

    (ch16, ln)
}

//fn bytes_to_str(bytes: &[u8]) -> String {
//    let res = bytes.iter().map(|&s| s as char).collect::<String>();
//    String::from_utf8(bytes.to_vec()).unwrap()
//}

#[derive(Debug, Clone)]
pub struct DicEntry {
    pub original: String,
    pub lc_attr: u16,
    pub rc_attr: u16,
    pub posid: u16,
    pub wcost: i16,
    pub feature: String,
}

#[derive(Debug)]
pub struct CharProperty {
    pub mmap: Mmap,
    pub category_names: Vec<String>,
    pub offset: usize,
}

impl CharProperty {
    pub fn open(dic_path: &str) -> Result<CharProperty, std::io::Error> {
        let file = File::open(dic_path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let mut category_names: Vec<String> = Vec::new();
        let num_categories = unpack_u32(&mmap, 0);
        for i in 0..num_categories {
            category_names.push(unpack_string(&mmap, (4 + i * 32) as usize));
        }

        let char_property = CharProperty {
            mmap: mmap,
            category_names: category_names,
            offset: (4 + num_categories * 32) as usize,
        };
        Ok(char_property)
    }

    pub fn get_char_info(&self, code_point: u16) -> (u32, u32, u32, u32, u32) {
        let v = unpack_u32(&self.mmap, self.offset + (code_point as usize) * 4);
        (
            (v >> 18) & 0b11111111,   // default_type
            v & 0b111111111111111111, // type
            (v >> 26) & 0b1111,       // char count
            (v >> 30) & 0b1,          // group
            (v >> 31) & 0b1,          // invoke
        )
    }

    pub fn get_group_length(&self, s: &[u8], default_type: u32, max_count: u32) -> usize {
        // aggregate same char types and return length
        let mut i: usize = 0;
        let mut char_count: u32 = 0;
        while i < s.len() {
            let (ch16, ln) = utf8_to_ucs2(s, i);
            // default_type, type, count, group, invoke
            let (_, t, _, _, _) = self.get_char_info(ch16);

            if ((1 << default_type) & t) != 0 {
                i += ln;
                char_count += 1;
                if max_count != 0 && max_count == char_count {
                    break;
                }
            } else {
                break;
            }
        }
        i
    }

    pub fn get_count_length(&self, s: &[u8], count: u32) -> usize {
        // get char count bytes length
        let mut i: usize = 0;
        for _ in 0..count {
            let (_ch16, ln) = utf8_to_ucs2(s, i);
            i += ln;
        }
        i
    }

    pub fn get_unknown_lengths(&self, s: &[u8]) -> (u32, Vec<usize>, bool) {
        // get unknown word bytes length vector
        let mut ln_vec: Vec<usize> = Vec::new();
        let (ch16, _ln) = utf8_to_ucs2(s, 0);
        let (default_type, _, count, group, invoke) = self.get_char_info(ch16);
        if group != 0 {
            ln_vec.push(self.get_group_length(s, default_type, count));
        } else {
            ln_vec.push(self.get_count_length(s, count));
        }

        // type, vector of length, invoke always flag
        (default_type, ln_vec, invoke == 1)
    }
}

#[derive(Debug)]
pub struct MeCabDic {
    pub mmap: Mmap,
    pub dic_size: u32,
    lsize: u32,
    rsize: u32,
    da_offset: u32,
    token_offset: u32,
    feature_offset: u32,
}

impl MeCabDic {
    pub fn open(dic_path: &str) -> Result<MeCabDic, std::io::Error> {
        let file = File::open(dic_path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let dic_size = unpack_u32(&mmap, 0) ^ 0xef718f77;
        let _version = unpack_u32(&mmap, 4);
        let _dictype = unpack_u32(&mmap, 8);
        let _lexsize = unpack_u32(&mmap, 12);
        let lsize = unpack_u32(&mmap, 16);
        let rsize = unpack_u32(&mmap, 20);
        let dsize = unpack_u32(&mmap, 24);
        let tsize = unpack_u32(&mmap, 28);
        let _fsize = unpack_u32(&mmap, 32);
        let _dummy = unpack_u32(&mmap, 36);

        let dic = MeCabDic {
            mmap: mmap,
            dic_size: dic_size,
            lsize: lsize,
            rsize: rsize,
            da_offset: 72,
            token_offset: 72 + dsize,
            feature_offset: 72 + dsize + tsize,
        };
        Ok(dic)
    }

    fn base_check(&self, idx: u32) -> (i32, u32) {
        let i: usize = (self.da_offset + idx * 8) as usize;
        (unpack_i32(&self.mmap, i), unpack_u32(&self.mmap, i + 4))
    }

    pub fn exact_match_search(&self, s: &[u8]) -> i32 {
        let mut v = -1;
        let mut p: u32;

        let (mut b, _) = self.base_check(0);
        for (_i, &item) in s.iter().enumerate() {
            p = (b + (item as i32)) as u32 + 1;
            let (base, check) = self.base_check(p);
            if b == (check as i32) {
                b = base;
            } else {
                return v;
            }
        }

        p = b as u32;
        let (n, check) = self.base_check(p);
        if b == (check as i32) && n < 0 {
            v = -n - 1;
        }
        v
    }

    pub fn common_prefix_search(&self, s: &[u8]) -> Vec<(i32, usize)> {
        let mut results: Vec<(i32, usize)> = Vec::new();
        let mut p: u32;

        let (mut b, _) = self.base_check(0);
        for (i, &item) in s.iter().enumerate() {
            p = b as u32;
            let (n, check) = self.base_check(p);
            if b == (check as i32) && n < 0 {
                results.push((-n - 1, i as usize));
            }
            p = (b + (item as i32)) as u32 + 1;
            let (base, check) = self.base_check(p);
            if b == (check as i32) {
                b = base;
            } else {
                return results;
            }
        }
        p = b as u32;

        let (n, check) = self.base_check(p);
        if b == (check as i32) && n < 0 {
            results.push((-n - 1, s.len() as usize));
        }

        results
    }

    fn get_entries_by_index(&self, idx: u32, count: u32, s: &str) -> Vec<DicEntry> {
        let mut results: Vec<DicEntry> = Vec::new();
        for i in 0..count {
            let lc_attr = unpack_u16(&self.mmap, (self.token_offset + (idx + i) * 16) as usize);
            let rc_attr = unpack_u16(
                &self.mmap,
                (self.token_offset + (idx + i) * 16 + 2) as usize,
            );
            let posid = unpack_u16(
                &self.mmap,
                (self.token_offset + (idx + i) * 16 + 4) as usize,
            );
            let wcost = unpack_i16(
                &self.mmap,
                (self.token_offset + (idx + i) * 16 + 6) as usize,
            );
            let feature = unpack_u32(
                &self.mmap,
                (self.token_offset + (idx + i) * 16 + 8) as usize,
            );
            let feature = unpack_string(&self.mmap, (self.feature_offset + feature) as usize);
            results.push(DicEntry {
                original: s.to_string(),
                lc_attr: lc_attr,
                rc_attr: rc_attr,
                posid: posid,
                wcost: wcost,
                feature: feature,
            });
        }

        results
    }

    fn get_entries(&self, result: u32, s: &str) -> Vec<DicEntry> {
        let index = result >> 8;
        let count = result & 0xFF;
        self.get_entries_by_index(index, count, s)
    }

    pub fn lookup(&self, s: &[u8]) -> Vec<DicEntry> {
        let mut results: Vec<DicEntry> = Vec::new();
        for (result, len) in self.common_prefix_search(s).iter() {
            let index = (*result >> 8) as u32;
            let count = (result & 0xFF) as u32;
            let mut new_results =
                self.get_entries_by_index(index, count, str::from_utf8(&s[..*len]).unwrap());
            results.append(&mut new_results);
        }
        results
    }

    pub fn lookup_unknowns(&self, s: &[u8], cp: &CharProperty) -> (Vec<DicEntry>, bool) {
        let (default_type, ln_vec, invoke) = cp.get_unknown_lengths(s);
        let index = self.exact_match_search(cp.category_names[default_type as usize].as_bytes());
        let mut results: Vec<DicEntry> = Vec::new();
        for i in ln_vec {
            let mut new_results = self.get_entries(index as u32, str::from_utf8(&s[..i]).unwrap());
            results.append(&mut new_results);
        }
        (results, invoke)
    }
}

#[derive(Debug)]
pub struct Matrix {
    pub mmap: Mmap,
    pub lsize: usize,
    pub rsize: usize,
}

impl Matrix {
    pub fn open(dic_path: &str) -> Result<Matrix, std::io::Error> {
        let file = File::open(dic_path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let lsize = unpack_u16(&mmap, 0) as usize;
        let rsize = unpack_u16(&mmap, 2) as usize;

        let matrix = Matrix {
            mmap: mmap,
            lsize: lsize,
            rsize: rsize,
        };
        Ok(matrix)
    }

    pub fn get_trans_cost(&self, id1: u16, id2: u16) -> i32 {
        let id1 = id1 as usize;
        let id2 = id2 as usize;

        unpack_i16(&self.mmap, ((id2 * self.lsize + id1) * 2 + 4) as usize) as i32
    }
}

#[test]
fn test_dic_open() {
    assert!(
        MeCabDic::open("/something/wrong/path/sys.dic").is_err(),
        "Error not occured."
    );
    let result = MeCabDic::open("/var/lib/mecab/dic/ipadic-utf8/sys.dic");
    assert!(!result.is_err(), "Can't open dict file.");
    let sys_dic = result.unwrap();
    assert_eq!(sys_dic.dic_size, 49199027);
}

#[test]
fn test_char_property() {
    let cp = CharProperty::open("/var/lib/mecab/dic/ipadic-utf8/char.bin").unwrap();

    assert_eq!(
        cp.category_names,
        vec![
            "DEFAULT",
            "SPACE",
            "KANJI",
            "SYMBOL",
            "NUMERIC",
            "ALPHA",
            "HIRAGANA",
            "KATAKANA",
            "KANJINUMERIC",
            "GREEK",
            "CYRILLIC"
        ]
    );

    // (default_type, type, length, group, invoke)
    assert_eq!(cp.get_char_info(0), (0, 1, 0, 1, 0)); // DEFAULT
    assert_eq!(cp.get_char_info(0x20), (1, 2, 0, 1, 0)); // SPACE
    assert_eq!(cp.get_char_info(0x09), (1, 2, 0, 1, 0)); // SPACE
    assert_eq!(cp.get_char_info(0x6f22), (2, 4, 2, 0, 0)); // KANJI 漢
    assert_eq!(cp.get_char_info(0x3007), (3, 264, 0, 1, 1)); // SYMBOL
    assert_eq!(cp.get_char_info(0x31), (4, 16, 0, 1, 1)); // NUMERIC 1
    assert_eq!(cp.get_char_info(0x3042), (6, 64, 2, 1, 0)); // HIRAGANA あ
    assert_eq!(cp.get_char_info(0x4e00), (8, 260, 0, 1, 1)); // KANJINUMERIC 一
}

#[test]
fn test_get_trans_cost() {
    let matrix = Matrix::open("/var/lib/mecab/dic/ipadic-utf8/matrix.bin").unwrap();
    assert_eq!(matrix.get_trans_cost(555, 1283), 340);
    assert_eq!(matrix.get_trans_cost(10, 1293), -1376);
}

fn assert_entry(e: &DicEntry, lc_attr: u16, rc_attr: u16, posid: u16, wcost: i16) {
    assert_eq!(e.lc_attr, lc_attr);
    assert_eq!(e.rc_attr, rc_attr);
    assert_eq!(e.posid, posid);
    assert_eq!(e.wcost, wcost);
}

#[test]
fn test_lookup() {
    let sys_dic = MeCabDic::open("/var/lib/mecab/dic/ipadic-utf8/sys.dic").unwrap();
    let sb = "すもももももももものうち".as_bytes();

    let r = sys_dic.common_prefix_search(&sb[0..]);
    assert_eq!(r.len(), 3);
    assert_eq!(r[0], (8849415, 3));
    assert_eq!(r[1], (9258497, 6));
    assert_eq!(r[2], (9259009, 9));

    let entries = sys_dic.lookup(sb);
    assert_eq!(entries.len(), 9);
    assert_entry(&entries[0], 560, 560, 30, 10247);
    assert_entry(&entries[1], 879, 879, 32, 11484);
    assert_entry(&entries[2], 777, 777, 31, 9683);
    assert_entry(&entries[3], 602, 602, 31, 9683);
    assert_entry(&entries[4], 601, 601, 31, 9683);
    assert_entry(&entries[5], 1285, 1285, 38, 10036);
    assert_entry(&entries[6], 11, 11, 10, 9609);
    assert_entry(&entries[7], 763, 763, 31, 9412);
    assert_entry(&entries[8], 1285, 1285, 38, 7546);
}

#[test]
fn test_lookup_unknowns() {
    let unk_dic = MeCabDic::open("/var/lib/mecab/dic/ipadic-utf8/unk.dic").unwrap();
    let cp = CharProperty::open("/var/lib/mecab/dic/ipadic-utf8/char.bin").unwrap();

    assert_eq!(unk_dic.exact_match_search(b"SPACE"), 9729);

    let (entries, invoke) = unk_dic.lookup_unknowns("１９６７年".as_bytes(), &cp);
    assert_eq!(entries.len(), 1);
    assert_eq!(invoke, true);
    assert_eq!(entries[0].original, "１９６７");
}
