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
use awabi::tokenizer;
use clap::{App, Arg};
use std::io::{self, Read};

fn print_tokens(tokens: &Vec<(String, String)>) {
    for t in tokens.iter() {
        println!("{}\t{}", t.0, t.1);
    }
    println!("EOS");
}

fn main() {
    let app = App::new("awabi").arg(
        Arg::with_name("nbest")
            .help("output N best results")
            .short("N")
            .long("nbest")
            .takes_value(true),
    );

    let matches = app.get_matches();
    let mut nbest = 1;
    if let Some(n_best) = matches.value_of("nbest") {
        nbest = n_best.parse().unwrap();
    }

    let mut lines = String::new();
    io::stdin().read_to_string(&mut lines).unwrap();
    lines = lines.trim_end().to_string();

    let tokenizer = tokenizer::Tokenizer::new(None).unwrap();
    for s in lines.split("\n") {
        if nbest == 1 {
            print_tokens(&tokenizer.tokenize(s));
        } else {
            for tokens in tokenizer.tokenize_n_best(s, nbest).iter() {
                print_tokens(tokens);
            }
        }
    }
}
