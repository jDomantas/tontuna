# Tontuna programming language

This is an entry to the [first langjam](https://github.com/langjam/jam0001).
Theme of the contest was "first class comments".

There were updates since the submission (added more language features and a
language server). If you want to see the submitted version take a look at
[langjam-submission](https://github.com/jDomantas/tontuna/tree/langjam-submission)
tag.


## Installation

There are three components in this repository:

1. Tontuna interpreter

    You need to have rust tooling installed (rustc, cargo). The required rust
    version is 1.53.0.

    You can install the `tontuna` interpreter from source using cargo. This will put
    `tontuna` in your `$PATH` so you can play around with it.

    ```shell
    cargo install --path tontuna
    ```

    If you don't want to install the binary, you can instead run it from the repo
    using `cargo run`:

    ```shell
    cargo run -p tontuna -- args to tontuna executable
    ```

    `tontuna` takes one parameter - the source file to run. Programs can only print
    to stdout (or stderr by crashing with a custom error message). You can redirect
    stdout to a file by adding `--output path` parameter.

    Usage examples:

    ```shell
    # invoke installed interpreter on doc-gen example
    tontuna programs/doc-gen/main.tnt

    # invoke installed interpreter on doc-gen example
    # and redirect output to a file
    tontuna programs/doc-gen/main.tnt --output output.md

    # build and invoke interpreter on doc-gen example (without installing)
    cargo run -p tontuna --release -- programs/doc-gen/main.tnt
    ```

2. Tontuna language server

    Just like the interpreter, language server can be installed from source
    using cargo:

    ```shell
    cargo install --path tontuna-lsp
    ```

    Language server implements only syntax highlighting and live error display.

3. Tontuna extension for vscode

    This extension can also be installed from vscode marketplace.

    Extension expects to find `tontuna-lsp` in your `$PATH`. If you installed
    the language server somewhere else you can change the path in extension
    settings.

    To build and install extension from source you will need npm 6.9.0 and vsce
    1.83.0.

    ```shell
    cd tontuna-vscode
    npm install
    vsce package
    code --install-extension tontuna-vscode-0.0.1.vsix
    ```

If you want to hack on the interpreter, you can run all of the tests (including
test and demo programs) with cargo:

```shell
cargo test
```


## Language

The base language is a dynamically typed statement-oriented language, with some
bits of syntax stolen from Rust. If you have read
[Crafting Interpreters](https://craftinginterpreters.com/) then it should feel
very familiar.

To spice it up and be on topic it has a few special features:

1. Comments are part of program's syntax tree. They can only appear in statement
positions though.
2. Programs can introspect their own source code (and thus read the comments).
This allows some interesting features to be implemented as libraries. And also
conveniently makes demo programs small and self-contained.
3. Comments can contain embedded code snippets (think of Rust's doc tests, but
anywhere). The snippets are not run by default, but they must be syntactically
correct and are available through code introspection.
4. Programs can evaluate code dynamically though an eval-like api. The exciting
thing that this opens up is that programs can run the code snippets embedded
in their own comments.


## Demo programs

There are three demo programs in this repo:

1. Documentation generator.

    This one generates its own markdown documentation by inspecting top-level
    comments and functions.

    [readme](./programs/doc-gen/README.md)

2. Literate programming demo.

    This program is written in a literate style (enabled by a little synctactic
    feature). It prints out its own code formatted as a nice markdown document.

    [readme](./programs/literate/README.md)

3. Documentation tests.

    This program implements documentation tests (like in rust) by looking
    through its own comments and running embedded code.

    [readme](./programs/doc-test/README.md)

There are also [a bunch of test programs](./programs/test-cases). Language
features are not documented anywhere, so you might want to take a look there to
see what's available.
