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
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use regex::Regex;

pub fn find_mecabrc() -> Option<String> {
    for path in vec!["/usr/local/etc/mecabrc", "/etc/mecabrc"] {
        if Path::new(path).exists() {
            return Some(path.to_string());
        }
    }
    None
}

pub fn rc_map(path: &str) -> Result<HashMap<String, String>, io::Error> {
    let mut rc: HashMap<String, String> = HashMap::new();
    let re = Regex::new(r"^(\S+)\s*=\s*(\S+)").unwrap();

    let f = File::open(path)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(caps) = re.captures(&line) {
            rc.insert(caps[1].to_string(), caps[2].to_string());
        }
    }

    Ok(rc)
}

pub fn get_dic_path(rc_map: &HashMap<String, String>, filename: &str) -> String {
    let dirname = &rc_map[&String::from("dicdir")];
    let mut s = String::from(dirname);
    s.push_str("/");
    s.push_str(filename);
    s
}

#[test]
fn test_mecabrc() {
    let rc_map = rc_map(&find_mecabrc().unwrap()).unwrap();
    assert_ne!(rc_map.get(&String::from("dicdir")), None);
}
