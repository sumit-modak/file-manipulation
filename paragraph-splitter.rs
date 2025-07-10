#!/usr/bin/env -S cargo +nightly -Zscript

//! Splits a single line paragraph into multiple lines

use std::io::Write;

// this is a blocking implementation
fn main() {
    let l = std::env::args()
        .nth(1)
        .expect("Please enter max character length")
        .parse::<usize>()
        .expect("Character length should be in numbers");
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    writeln!(out, "You are inside paragraph splitter shell").unwrap();

    loop {
        out.flush().unwrap();
        // user input buffer
        let mut input = String::new();
        // taking user input
        std::io::stdin().read_line(&mut input).unwrap();
        // creating output buffer
        let mut output = String::new();
        let mut line = String::with_capacity(l + 2);
        for word in input.split_whitespace() {
            if line.len() + word.len() <= l {
                line.push_str(word);
                line.push(' ');
            } else {
                line.pop();
                line.push('\n');
                output.push_str(&line);
                line.clear();
                line.push_str(word);
                line.push(' ');
            }
        }
        output.push_str(&line);
        line.clear();
        if !input.is_empty() {
            std::process::Command::new("wl-copy")
                .arg(&output)
                .output()
                .expect("failed to copy");
        }
        writeln!(out, "{output}\n").unwrap();
        out.flush().unwrap();
    }
}
