# Clua - Lua to C Compiler

A Lua compiler written in Rust that translates Lua code into C and generates executables.

## Overview

Clua is a specialized Lua compiler that converts Lua source code into C code and then compiles it into an executable. It's designed to be simple and efficient, focusing on basic Lua functionality.

## Features

- Converts Lua code to C
- Handles basic print statements
- AST-based code generation
- Error reporting and validation
- Automatic cleanup of temporary files

## Project Structure

- `src/main.rs`: Entry point and CLI handling
- `src/ast.rs`: Abstract Syntax Tree definitions
- `src/lex.rs`: Lexical analysis and tokenization
- `src/l2c.rs`: Lua to C code generation
- `src/check.rs`: Code validation and error checking

## Requirements

- Rust (latest stable version)
- Clang compiler
- Linux environment

## Usage

```bash
# Basic usage
clua <filename>.lua

# Show help
clua --help
clua -h
```

## Example

Create a file `test.lua`:
```lua
print("Hello, World!")
```

Compile and run:
```bash
clua test.lua
./test.out
```

## Current Limitations

- Only supports basic print statements with string literals
- No support for variables yet
- Limited error handling
- Single file compilation only (directory support coming soon)

## License

See the LICENSE file for details.

## Note

This project is made specially for Akshat and not intended for outside use.