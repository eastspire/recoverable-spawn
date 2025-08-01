<center>

## recoverable-spawn

[![](https://img.shields.io/crates/v/recoverable-spawn.svg)](https://crates.io/crates/recoverable-spawn)
[![](https://img.shields.io/crates/d/recoverable-spawn.svg)](https://img.shields.io/crates/d/recoverable-spawn.svg)
[![](https://docs.rs/recoverable-spawn/badge.svg)](https://docs.rs/recoverable-spawn)
[![](https://github.com/crates-dev/recoverable-spawn/workflows/Rust/badge.svg)](https://github.com/crates-dev/recoverable-spawn/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/recoverable-spawn.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/recoverable-spawn/)

[Api Docs](https://docs.rs/recoverable-spawn/latest/recoverable_spawn/)

> A thread that supports automatic recovery from panics, allowing threads to restart after a panic. Useful for resilient and fault-tolerant concurrency in network and web programming.

## Installation

To use this crate, you can run cmd:

```shell
cargo add recoverable-spawn
```

## Use

### recoverable_spawn

```rust
use recoverable_spawn::*;

let msg: &str = "test";
let res: SyncSpawnResult = recoverable_spawn(move || {
    panic!("{}", msg);
});
let res: SyncSpawnResult = recoverable_spawn_with_error_handle(
    move || {
        panic!("{}", msg);
    },
    |err| {
        println!("handle error => {}", err);
    },
);
```

### recoverable_spawn_with_error_handle

```rust
use recoverable_spawn::*;

let msg: &str = "test";
let res: SyncSpawnResult = recoverable_spawn_with_error_handle(
    move || {
        panic!("{}", msg);
    },
    |err| {
        println!("handle error => {}", err);
    },
);
```

### async_recoverable_spawn

```rust
use recoverable_spawn::*;

let msg: &str = "test";
let res: AsyncSpawnResult = async_recoverable_spawn(move || async move {
    panic!("{}", msg);
});
```

### async_recoverable_spawn_catch

```rust
use recoverable_spawn::*;

let msg: &str = "test";
let res: AsyncSpawnResult = async_recoverable_spawn_catch(
    move || async move {
        panic!("{}", msg);
    },
    move |err| async move {
        println!("handle error => {}", err);
    },
);
```

### async_recoverable_spawn_catch_finally

```rust
use recoverable_spawn::*;

let msg: &str = "test";
let res: AsyncSpawnResult = async_recoverable_spawn_catch_finally(
    move || async move {
        panic!("{}", msg);
    },
    move |err| async move {
        println!("handle error => {}", err);
        panic!("{}", err);
    },
    move || async move {
        println!("finally");
    },
);
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
