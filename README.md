# Tontuna programming language

This is an entry to the [first langjam](https://github.com/langjam/jam0001).
Theme of the contest was "first class comments".


## Installation

You need to have rust tooling installed (rustc, cargo). The required rust
version is 1.53.0.

You can install the `tontuna` interpreter from source using cargo. This will put
`tontuna` in your `$PATH` so you can play around with it.

```
cargo install --path tontuna
```

If you don't want to install the binary, you can instead run it from the repo
using `cargo run`:

```
cargo run -p tontuna -- args to tontuna executable
```

`tontuna` takes one parameter - the source file to run. Programs can only print
to stdout (or stderr by crashing with a custom error message). You can redirect
stdout to a file by adding `--output path` parameter.


## Language

The base language is a dynamically typed statement-oriented language, with some
bits of syntax stolen from Rust. If you have read
[Crafting Interpreters](https://craftinginterpreters.com/) then it should feel
very familiar.

To spice it up and be on topic it has a few special features:

1. Comments are part of program's syntax tree. They can only appear in statement
positions though.
2. Programs can introspect their own source code (and thus read the comments).
This allows some interesting features implementable as libraries. And also
conveniently makes demo programs small and self-contained.
3. Comments can contain embedded code snippets (think of Rust's doc tests, but
anywhere). The snippets are not run by default, but they must be syntactically
correct and are available through code introspection.
4. UNFINISHED: programs can evaluate code dynamically though an eval-like api.
The exciting thing that is opens up is that they can run the code snippets that
are embedded in its own comments.


## Demo programs

There's three demo programs in this repo:

1. Documentation generator.

    This one generates its own markdown documentation by inspecting top-level
    comments and functions.

    [readme](./programs/doc-gen/README.md)

2. Literate programming demo.

    This program is written in a literate style (enabled by a little synctactic
    feature). It prints out its own code formatted as a nice markdown document.

    [readme](./programs/literate/README.md)

3. Unfinished: documentation tests.

    [readme](./programs/doc-test/README.md)

There are also [a bunch of test programs](./programs/test-cases). Language
features are not documented anywhere, so you might want to take a look there to
see what's available.
