mod assembler;
mod compiler;
mod linker;

use crate::compiler::compile::compile;
use assembler::assembler;
use linker::linker;
use std::fs;

const GOAL: &str = "target/result/goal";

fn main() {
    let input = include_str!("../goal/variable.ra");

    let compiler_result = compile(input);
    let object_result = assembler(compiler_result);
    let elf_result = linker(object_result);

    fs::write(GOAL, elf_result).expect("ERROR: write");
    println!("wrote `{}`", GOAL)
}
