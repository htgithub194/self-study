# Async Await


### Concurency vs Paralellism


![concurency_vs_paralellism](images/concurency_vs_paralellism.drawio.svg "concurency_vs_paralellism")


### Runtime


The most popular runtime for Rust is *Tokio runtime*


Runtime provide mechanisms for:
* Store list of tasks run in concurrency
* Which task will be run next
* When to re-invoke a pending task
* I/O functions for async programming
...


```rust
// example from https://tokio.rs/tokio/tutorial/spawning
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}
```


#### Note for #[tokio::main]

```rust
#[tokio::main]
async fn main() {
    code_inside_main_fn();
}

// is equivalent to

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        code_inside_main_fn();
    })
}
```


#### Executor & Waker

![executor](images/executor.drawio.svg "executor")



### Async Await key words





### Future


### The big picture
