# Learning Rust

## Commands to know

- `cargo run` - Run the Rust program
- `rustc <path/to/rust/script>` - Compile a single Rust script

## Processes

- **Differences between using `output` and `spawn` when creating child processes:**
  - With `output`, STDOUT and STDERR are captured (i.e., they can be used with pipes). With `spawn`, STDOUT and STDERR are inherited (i.e., they are set to whatever the parent's STDOUT and STDERR are).
