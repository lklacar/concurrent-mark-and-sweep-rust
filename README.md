# Concurrent Mark and Sweep Garbage Collector written in Rust

## Introduction

This is an example of a concurrent mark and sweep garbage collector written in Rust.
It is intended to be used in a language runtime where you have complete control over the
memory layout (stack and heap).
Throughput is dynamically calculated so the GC takes 1% of the time the program is running.

It runs two threads concurrently:

- One for running the program
- One for Mark and Sweep garbage collection

## Usage

To run the example, simply run `cargo run` in the root directory of the project.

## License

This project is licensed under the MIT license. See the LICENSE file for more details.