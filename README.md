# Rust WDM Driver Template [![Rust](https://img.shields.io/badge/Language-Rust-CE412B?logo=rust&logoColor=white)](https://www.rust-lang.org)

## Purpose
A minimal **Windows Driver Model (WDM)** project template that demonstrates how to configure a Rust crate to build a functional kernel-mode driver.

## Requirements
- **Visual Studio** with "Desktop development with C++" workload.
- **Windows Driver Kit (WDK)** 22H2 or newer.
- **Clang (LLVM)**: Required for `bindgen` to generate Rust bindings from WDK headers.
- **Cargo-make**: Task runner used for the build pipeline (`cargo install cargo-make`).
- **Rust Toolchain**: Nightly is often recommended for experimental features like `alloc_error_handler`, though many WDK tasks now work on Stable.

## Project Structure

- **`.cargo/config.toml`** Specifies custom build flags, such as linking the C Runtime (CRT) statically and defining the target architecture.

- **`src/lib.rs`** The main source file containing the `DriverEntry` (initialization) and `DriverUnload` routines.

- **`build.rs`** The build script that uses the `wdk-build` crate to locate WDK headers and configure the linker for kernel mode.

- **`Cargo.toml`** The manifest file. For drivers, this typically sets the crate-type to `["cdylib"]`.

- **`Cargo.lock`** Ensures reproducible builds by locking the exact versions of all dependencies.

- **`Makefile.toml`** Configuration for `cargo-make`. It extends the default `wdk-build` logic to automate signing, inf-stamping, and package creation.

- **`wdm_driver_example_rs.inx`** The architecture-independent template used to generate the final `.inf` file for driver installation.