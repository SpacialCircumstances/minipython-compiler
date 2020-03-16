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

TODO
