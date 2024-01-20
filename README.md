# Interpreters-Compilers101
Experiments while reading the book Crafting Interpreters, building a toy compiler/interpreter in Rust

##  Break it down into beginner-friendly steps.
 
We'll use a simple arithmetic language as an example.

- Define the Language: Before you start writing a compiler, you need to define the language that you want to compile. For our example, let's say our language can evaluate arithmetic expressions like 2 + 3 * 4.

- Tokenization (Lexical Analysis): The first step in a compiler is to break the input into "tokens". Each token represents a meaningful chunk of the input. For our language, we might have tokens for numbers, plus signs, minus signs, etc.
```
#[derive(Debug)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}
```

- Parsing (Syntactic Analysis): The next step is to take the list of tokens and build a "parse tree". This represents the structure of the program. For our language, the parse tree for 2 + 3 * 4 might look like this:

```
enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
}
```

- Code Generation: The final step is to walk the parse tree and generate code. For our language, we might generate bytecode for a stack-based virtual machine. The bytecode for 2 + 3 * 4 might look like this:
Each of these steps is a big topic in itself, and there are many details I've left out for simplicity. But this should give you a rough idea of the steps involved in writing a compiler.

```
enum Bytecode {
    Push(i32),
    Add,
    Subtract,
    Multiply,
    Divide,
}
```
There are also many great resources available if you want to learn more. The book "Crafting Interpreters" by Bob Nystrom is a great place to start, and it's available for free online. There's also the "Rust Programming Language" book by Steve Klabnik and Carol Nichols, which is a great resource for learning Rust.

