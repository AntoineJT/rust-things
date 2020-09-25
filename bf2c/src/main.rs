use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Error: No input file specified!
Format: {} <input_file>", &args[0]);
        exit(1);
    }
    let bf_file = &args[1];

    let mut code = String::new();
    code.push_str("#include <stdio.h>
#include <stdlib.h>
int main() {
char *p = calloc(30000, sizeof(char));
");

    let bf_content = read_whole_file(bf_file);
    for chr in bf_content.as_slice() {
        match *chr as char {
            '>' => code.push_str("++p;\n"),
            '<' => code.push_str("--p;\n"),
            '+' => code.push_str("++*p;\n"),
            '-' => code.push_str("--*p;\n"),
            '.' => code.push_str("putchar(*p);\n"),
            ',' => code.push_str("*p = getchar();\n"),
            '[' => code.push_str("while (*p) {\n"),
            ']' => code.push_str("}\n"),
            _ => {},
        }
    }

    code.push_str("free(p);\n}");
    let out_fname = format!("{}.c", remove_extension(String::from(bf_file)));
    write_whole_file(out_fname, code);
}

// this function has been stolen from another of my projects: shell-compiler
fn remove_extension(filename: String) -> String {
    let mut fname = filename.clone();
    if !fname.contains('.') {
        return fname
    }
    loop {
        let c = fname.pop().unwrap();
        if c == '.' {
            break
        }
    }
    fname
}

// read and write functions has been taken from another of my projects: the weird-archiver
fn read_whole_file(filename: &str) -> Vec<u8> {
    let mut input = File::open(filename)
        .expect(format!("No {} file found!", filename).as_str());
    let mut buffer: Vec<u8> = Vec::new();
    input.read_to_end(&mut buffer)
        .expect("An error occured during original file reading");
    buffer
}

fn write_whole_file(filename: String, content: String) {
    let mut file = File::create(filename).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
