## Crates

[cargo-tarpaulin](https://crates.io/crates/cargo-tarpaulin)\
`cargo install cargo-tarpaulin`
- `cargo tarpaulin --ignore-tests`

[cargo-audit](https://crates.io/crates/cargo-audit)\
`cargo install cargo-audit`
- `cargo audit`

[cargo-edit](https://crates.io/crates/cargo-edit)\
`cargo install cargo-edit`
- `cargo add actix-web`

[cargo-expand](https://crates.io/crates/cargo-expand)\
`cargo install cargo-expand`
- `cargo expand`

[sqlx-cli](https://crates.io/crates/sqlx-cli)\
`cargo install sqlx-cli --no-default-features --features postgres`
- `cargo sqlx prepare -- --lib`
- `sqlx migrate add migration_name`
- `sqlx migrate run`

[bunyan](https://crates.io/crates/bunyan)\
`cargo install bunyan`
- `TEST_LOG=true cargo test health_check_works | bunyan`

[cargo-watch](https://crates.io/crates/cargo-watch)\
`cargo install cargo-watch`
- `cargo watch -x check -x test -x "run | bunyan"`