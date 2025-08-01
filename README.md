# Mini Compiler

#### It includes a **lexer**, **parser**, and **code generation** to an intermediate representation, similar to three-address code. 
---

## Features

- ✅ Lexical analysis with a custom tokenizer  
- ✅ Recursive descent parser for expressions, statements, and functions  
- ✅ Symbol table support with lexical scoping  
- ✅ Type-aware parsing for `int`, arrays, and functions  
- ✅ IR-style code generation (three-address style)  
- ✅ Support for:
  - Variable declarations and assignments  
  - Array indexing and manipulation  
  - Control flow (`if`, `else`, `while`)  
  - Input/output (`read`, `print`)  
  - Function calls and returns  
---

## Valid Code Example

```rust
func main() {
    int i;
    int j;
    i = 0;
    while i < 2 {
        j = 0;
        while j < 3 {
            print(j);
            j = j + 1;
        }
        i = i + 1;
    }
}

```

## Intermediate Output
```
%func main()
%int i
%int j
%mov i, 0
while(%int temp0
%lt temp0, i, 2
cond(temp0))
%mov j, 0
while(%int temp1
%lt temp1, j, 3
cond(temp1))
%out j
%int temp2
%add temp2, j, 1
%mov j, temp2
%int temp3
%add temp3, i, 1
%mov i, temp3
%endfunc
```

## Usage

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)

### Run

```bash
git clone https://github.com/ManzanaNaranja/Rust-Compiler-Project.git
cd Rust-Compiler-Project
cargo run code.txt
