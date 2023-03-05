# :question: unknown-lang
[![CI](https://github.com/taxborn/unknown-lang/actions/workflows/ci.yml/badge.svg)](https://github.com/taxborn/unknown-lang/actions/workflows/ci.yml)
A programming language, in the works.

## Compiling
```bash
cargo run -- --file <file.ukl>
```
### Examples
There are a few examples right now, however since there is only a lexer, you can
only output the tokenization of the file for now. Those examples are in the 
[examples](./examples/) directory, and 

```bash
cargo run -- --file examples/mut.ukl -p
```
Pass through the `-p` or `--print-tokens` flag to print the tokens

## Grammar
The [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form)
grammar will be created as I work on the parser, and will be located at 
[grammar.ebnf](./grammar.ebnf).
