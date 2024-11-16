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

### License

This project is licensed under the [***Joay License***](LICENSE), with the following exception:

**Jaytirth Kundan**, the *author* and *creator* of this program, who lives in **India**, with the following emails:
- **joy0094@proton.me**
- **skub007@proton.me**

grants a special, unrestricted license to **Akshat Raj**, his friend, for full usage, modification, and distribution rights of this program. This exception applies **only** to **Akshat Raj**, and the terms of the Joay License apply to **all other users** and entities.

#### Identity Verification:

- **Jaytirth Kundan**, the **original author** of this project, can be uniquely identified as follows:
  - GitHub: [Jaytirth Kundan GitHub](https://github.com/skub007)
  - Email: **joy0094@proton.me** and **skub007@proton.me**
  - Residence: **India**
  - Public references and contributions: This project, along with other open-source projects at Oxumlabs git [here](https://github.com/Oxumlabs), can be found on GitHub. These references and contributions, can be traced back to the original author through commit history, repository ownership, and project documentation

- **Akshat Raj** is granted an exemption based on a **personal relationship** with **Jaytirth Kundan** and is the **only individual** who is not bound by the Joay License in full.

#### Important Conditions:

1. **Special License Exception**:
    - The license exception is **exclusive** to **Akshat Raj known by jaytirth kundan**. No one else is allowed to use this exception, and the standard Joay License applies to all other users.

2. **No Transferability**:
    - This exception is **non-transferable**. Akshat Raj **cannot** transfer or extend this special exception to others.

3. **Anti-Fraud Clause**:
    - Any individual or entity falsely claiming to be **Jaytirth Kundan** (the author) in order to bypass the terms of this license will have their claims considered **fraudulent** and **invalid**.
    - Identity verification is required for any disputes regarding ownership. Official verification can be done through **GitHub** and **email correspondence** linked above.

4. **Documentation and Proof of Ownership**:
    - As the **original creator**, **Jaytirth Kundan** maintains full control over the project, and this project can be verified by checking the **commit history**, **repository ownership**, and **public announcements** made by the author across platforms.
## Note

This project is made specially for Akshat and not intended for outside use.