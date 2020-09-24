use std::fs::File;
use std::io::{Read, Write};

fn main() {
    // TODO Get this by using program arguments
    let bf_file = "mandelbrot.bf";

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
    write_whole_file(File::create("mandelbrot.c").unwrap(), code);
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

fn write_whole_file(mut file: File, content: String) {
    file.write_all(content.as_bytes()).unwrap();
}
