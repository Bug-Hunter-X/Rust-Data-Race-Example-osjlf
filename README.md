# Rust Data Race Example

This repository demonstrates a simple example of a data race in Rust.  Data races occur when multiple threads or parts of a program access and modify the same data concurrently without proper synchronization. This can lead to unpredictable and erroneous results.

The `bug.rs` file contains the code exhibiting the data race. The `bugSolution.rs` file shows how to fix this using a mutex.