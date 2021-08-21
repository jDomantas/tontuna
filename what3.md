# Builtins

```rust
struct List(T) {
    fn push(value: T);
    fn get(index: int);
}

struct Comment {
    items: List(Comment | Code);
    fn whole_text() -> str;
}

struct Code {
    items: List(Comment | Code);
    fn exec(env: Env) -> RuntimeError | Env;
}

fn program_code();

struct ResolveError {
    fn str() -> str;
}

struct RuntimeError {
    fn str() -> str;
}

struct Env {
    fn is_defined(name: str) -> bool;
    fn get(name: str) -> any;
    fn define(name: str, value: any) -> Env;
}
```

# Doc generation

```rust
fn generate_docs() {
    let source = program_code();
    let previous_comment;
    println("---");
    for item in source {
        if let code: Code = item {
            let name = get_def_name(c);
            if name != nil {
                println();
                println("Function ", name);
                if previous_comment != nil {
                    println();
                    println(previous_comment.whole_text());
                }
                println();
                println("---");
            }
            previous_comment = nil;
        } else if let comment: Comment = item {
            previous_comment = comment;
        } else {
            panic("expected code or comment");
        }
    }
}
```

# Doctests

```rust
struct ChainedEnv {
    fn init(name, value, rest) {
        self.name = name;
        self.value = value;
        self.rest = rest;
    }

    fn is_defined(name) {
        if self.name == name {
            return true;
        } else if self.rest != nil {
            return self.rest.is_defined(name);
        } else {
            return false;
        }
    }

    fn get(name) {
        if self.name == name {
            return self.value;
        } else {
            return self.rest.get(name);
        }
    }

    fn define(name, value) {
        return ChainedEnv(name, value, self);
    }
}

struct EmptyEnv {
    fn is_defined(name) {
        return nil;
    }

    fn get(name) {
        panic("name is not defined");
    }

    fn define(name, value) {
        return ChainedEnv(name, value, self);
    }
}

fn doc_tests() {
    // checks only top level comments
    // code in comments must be standalone
    let source = program_code();
    for item in source {
        if let comment: Comment = item {
            for comment_item in comment.items {
                if let code: Code = comment_item {
                    let result = code.exec(EmptyEnv());
                    if let err: RuntimeError = result {
                        println("doc test failed: ", err.str());
                    } else {
                        panic("doc test passed");
                    }
                }
            }
        }
    }
}
```

# Literate programming

```rust
fn literate() {
    let source = program_code();
    for item in source {
        if let comment: Comment = item {
            for comment_item in comment.items {
                if let code: Code = comment_item {
                    let result = code.exec(EmptyEnv());
                    if let err: RuntimeError = result {
                        println("doc test failed: ", err.str());
                    } else {
                        panic("doc test passed");
                    }
                }
            }
        }
    }
}
```
