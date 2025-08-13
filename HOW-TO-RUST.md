# HOW TO RUST

## Create project

### Just a file

Create folder for the file and create file `main.rs`:

```rust
fn main() {
  println!("Hello, world!")
}
```

- Compile the file with `rustc main.rs`
- Run the file with `./main`

### Cargo

- Run `cargo new hello-cargo`

  - `cargo new hello-cargo --vcs none` without Git init

  - It makes a `hello-cargo` folder and necessary files

- Compile the project with `cargo build`

  - And run the project with `./target/debug/hello-cargo`

- Or compile and run the project with one command `cargo run`

- `cargo check` checks that program is executable without building so use it often
- `cargo build --release` for releases (makes `./target/release/hello-cargo` file with all the optimizations)

## Cargo dependencies

You add the dependencies you want to use in the `Cargo.toml` file:

```toml
[package]
name = "b_guessing-game"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5"
```

for example like this for random number

- Remember to run `cargo build` after adding dependencies

- `https://crates.io/` is where you can find the dependencies
- `cargo update` updates the dependencies

#### IMPORTANT

- `cargo doc --open` will build documentation provided by all your dependencies locally and open it in your browser
