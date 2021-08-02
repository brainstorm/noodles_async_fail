# Why U not Send, Noodles?

EDIT: Fixed [upstream in Noodles](https://github.com/zaeleus/noodles/issues/34).

```rust
error: future cannot be sent between threads safely
   --> src/main.rs:8:3
    |
8   |   tokio::spawn(async move {
    |   ^^^^^^^^^^^^ future created by async block is not `Send`
    | 
   ::: /Users/rvalls/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.9.0/src/task/spawn.rs:127:21
    |
127 |         T: Future + Send + 'static,
    |                     ---- required by this bound in `tokio::spawn`
    |
    = help: the trait `Send` is not implemented for `(dyn Future<Output = Result<bgzf::block::Block, std::io::Error>> + 'static)`
note: captured value is not `Send`
   --> src/main.rs:9:5
    |
9   |     reader.read_reference_sequences().await;
    |     ^^^^^^ has type `noodles_bam::AsyncReader<tokio::fs::File>` which is not `Send`

error: aborting due to previous error

error: could not compile `noodles_async_fail`

To learn more, run the command again with --verbose.
```
