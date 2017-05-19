# About this repository
- Uses standard `core` etc.
- Provides a minimal self-hosted `std` (see https://doc.rust-lang.org/book/no-stdlib.html)
- Our `std` referenced via `stage=1` in Xargo.toml (see https://github.com/japaric/xargo/pull/117)

Three projects, each with their own `Cargo.toml`:
1. `std_dep`. Depends only on `core`. Represents some low-level OS dependency that `std` builds on.
2. `std`. Depends on `core` and `std_dep`. Represents the real Rust `std`.
3. `app`. Depends on `core`, `std_dep` and `std`. Represents a real app.

Replacement of `std` is enabled via `app/Xargo.toml`:

    [dependencies]
    core = { }
    std = { path = "../libstd", stage = 1 }

# How to build
1. `cargo install xargo`
2. `cd app`
3. `xargo build`

        Compiling core v0.0.0 (file:///$HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src/libcore)
         Finished release [optimized] target(s) in 14.55 secs
        Compiling std_dep v0.1.0 (file:///$REPO/libstd_dep)
        Compiling std v0.1.0 (file:///$REPO/libstd)
         Finished release [optimized] target(s) in 0.19 secs
        Compiling libc v0.2.23
        Compiling std_dep v0.1.0 (file:///$REPO/libstd_dep)
        Compiling app v0.1.0 (file:///$REPO/app)
         Finished dev [unoptimized + debuginfo] target(s) in 1.34 secs
