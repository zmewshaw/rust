Macros vs Functions:
- Functions must declare the amount of parameters while macros have a variable number of parameters.
- Functions are called at runtime while macros are expanded before the program finishes compiling.
- Downside is increased complexity because you are writing code that writes other code.

2 Forms of Macros:
- Declarative
	- Most widely used form of macros.
	- Allow you to write something similar to a match expression.
- Procedural
	- Like functions in the sense that they take code as input, operate on that code, and produce code as output.
	- Declarative replace code with other code.

Everything else:
- adding `?` to the end of a statement will allow it to return whatever error it could create, otherwise execute correctly
- Limit generics to types that implement a trait with the syntax `<T: *trait*>`
- 1.  you can make a trait a return type with the syntax: `impl *trait*`
- 1.  **Lifetimes do not change the actual lifetime, they just specify the relationship**
- `'static` is a lifetime that specifies that a reference will live for the duration of the program
- Update your local toolchain with: `rustup self update` and `rustup update`
- Code can be checked without being compiled to significantly increase write-time with the command `cargo check`
- `rustfmt` is a code formatter that allows codebases to have a consistent coding style and avoid nitpicking during code reviews.
	- Can be used with `cargo fmt`
- `clippy` is a linter for Rust. It will detect code patterns that may lead to errors or are identified by the community as bad style.
	- Can be installed with the command: `rustup component add clippy`
	- Can be used with `cargo clippy`
- Update dependencies in `Cargo.toml` with the command: `cargo update`
- `cargo-outdated` is a program that helps you to identify your outdated dependencies
	- Can be installed with `cargo install -f cargo-outdated`
	- Can be used with `cargo outdated`
- If you need to keep using old dependencies for a project, you can scan them for vulnerabilities with `cargo-audit`
	- Can be installed with `cargo install -f cargo-audit`
	- Can be used with `cargo audit`