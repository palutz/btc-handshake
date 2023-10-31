# btc-handshake

![Rust Version][rustc-image]
[![crates.io][crate-image]][crate-link]
[![Documentation][docs-image]][docs-link]
[![Dependency Status][deps-image]][deps-link]


## How to run it

Download the Bitcoin core implementation (source or binary) and then run:
```
$ bitcoind -regtest -daemon
```

From the root folder of the project, run:
```
make run
```

and the program will make an handshake with the Bitcion Core node.


## Design
The program is hugely simplified.
For this test I decided to:
- reduce the dependencies to external libraries
- deal only with IPv4 addresses
- deal only with local nodes (hardcoded localhost address)
- Simplified management of the messages
- almost no error management

[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.60+-blue.svg
[crate-image]: https://img.shields.io/crates/v/{{project-name}}.svg
[crate-link]: https://crates.io/crates/{{project-name}}
[docs-image]: https://docs.rs/{{project-name}}/badge.svg
[docs-link]: https://docs.rs/{{project-name}}
[deps-image]: https://deps.rs/repo/github/palutz/lisp_interpreter_rs/status.svg
[deps-link]: https://deps.rs/repo/github/palutz/lisp_interpreter_rs

