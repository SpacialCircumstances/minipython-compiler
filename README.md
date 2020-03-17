# minipython-compiler

A slightly optimizing compiler for minipython (see [here](https://github.com/SpacialCircumstances/minipython-interpreter) for an interpreter).
Minipython code is parsed (correctly, with indentation, this time), then translated to a IR.
The translation stage applies some very basic optimizations and infers the creation time of variables.
From the IR, C code is generated (only depends on stdio.h).

## CLI

```
MiniPython compiler
Compiles MiniPython programs

USAGE:
    minipython-c.exe [OPTIONS] <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --out <FILE>    Sets the output file name

ARGS:
    <FILE>    Input file
```

## Limitations

The indentation-based syntax is fully supported this time (no comments at block ends needed).
Functions can only be declared at the top-level and cannot be nested.
Error reporting is very basic: No location information (after parser errors) and only the first error is usually reported.
The datatype used in C is `unsigned long long int` and is defined to be at least 64-bit in size. Still, for a language that has to encode everything as integers this is rather limiting.
Performance is quite good, as long as the C compiler is used with `-O3`. Not because the minipython compiler is smart, but because C compilers are *really* smart.

## Conclusion

Minipython was choosen by me as a simple project to learn how to write a compiler in Rust.
Overall, the experience was decently smooth. [LALRPOP](https://github.com/lalrpop/lalrpop) is a really great parser generator. It's decently easy to use, with good error messages, and thanks to cargo very well-integrated into the build process. The only caveat was that I had to write my own lexer, but that was necessary anyways, because I processed indentation in the lexer to let the parser stay Type-2. Writing a lexer took some time due to the complexity of parsing indentation, comments etc. correctly, but Rust turns out to be a language well suited to the task, thanks to the wide support of control flow (returns from loop etc.) and thanks to iterators.
At the same time, Rust's preference for mutable strings and slices makes writing the AST a bit more awkward, because it either requires keeping the entire source code just for the slices into it, frequent cloning, or string interning. I went with the last solution. While string interning is good for performance, it also makes printing, debugging and testing a bit harder. Cloning would have probably been the better choice, but Rust inspired me to be very performance-aware, even if it is really unneccessary. The usual caveat with premature optimization applies especially to Rust, a language that is very explicit about performance costs.
