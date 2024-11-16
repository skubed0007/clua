use l2c::l2c;
use lex::lex;
use std::path::Path;
use std::{env, fs, process::Command};
mod ast;
mod check;
mod l2c;
mod lex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_usage();
        return;
    }

    if args[1] == "--help" || args[1] == "-h" {
        print_help();
        return;
    }

    if fs::metadata(&args[1]).is_err() {
        eprintln!("{}: No such file or directory", &args[1]);
        return;
    }

    if fs::metadata(&args[1]).unwrap().is_dir() {
        dirhan(&args[1]);
    } else if fs::metadata(&args[1]).unwrap().is_file() {
        filehan(&args[1]);
    } else {
        eprintln!("Invalid path provided.");
    }
}

fn print_usage() {
    println!(
        "Usage: clua <filename>.lua\nCompiles a Lua file into a C file and generates an executable."
    );
}

fn print_help() {
    println!(
        "Clua - Lua Compiler written in Rust\nMade specially for Akshat and not for outside use\n\n\
        Usage:\n\
        clua <filename>.lua\n\n\
        Arguments:\n\
        <filename>        The Lua file to be compiled into a C program.\n\n\
        Options:\n\
        --help, -h        Show this help message and exit.\n\n\
        Example:\n\
        clua hello.lua     Compiles hello.lua into a C file and generates an executable."
    );
}

fn dirhan(path: &str) {
    println!("Compiling directory: {}\nNOT IMPLEMENTED YET", path);
}

fn filehan(path: &str) {
    println!("Compiling file: {}", path);
    match fs::read_to_string(&path) {
        Ok(code) => {
            let ast = lex(&code);
            let c_code = l2c(ast);
            let temp_c_file = format!("{}_temp.c", path);
            let output_file = format!("{}.out", path.trim_end_matches(".lua"));

            match fs::write(&temp_c_file, &c_code) {
                Ok(_) => {
                    let output = Command::new("clang")
                        .arg(&temp_c_file)
                        .arg("-o")
                        .arg(&output_file)
                        .output();

                    if output.is_err() || !output.as_ref().unwrap().status.success() {
                        eprintln!(
                            "Compilation failed: {}",
                            output
                                .as_ref()
                                .map(|o| String::from_utf8_lossy(&o.stderr).to_string())
                                .unwrap_or_else(|_| "Unknown error".to_string())
                        );
                        cleanup(&temp_c_file, &output_file);
                    } else {
                        println!("Compilation successful! Output: {}", output_file);
                    }
                }
                Err(e) => {
                    eprintln!("Error writing temporary file: {}", e);
                    cleanup(&temp_c_file, &output_file);
                }
            }
        }
        Err(e) => {
            eprintln!("{}: {}", path, e);
        }
    }
}

fn cleanup(temp_c_file: &str, output_file: &str) {
    if fs::remove_file(temp_c_file).is_ok() {
        println!("Temporary file {} removed.", temp_c_file);
    }
    if Path::new(output_file).exists() {
        if fs::remove_file(output_file).is_ok() {
            println!("Incomplete output file {} removed.", output_file);
        }
    }
}
