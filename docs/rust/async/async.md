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


Async Await key words are use to write asynchronous task.


```rust
async fn main {
    bar.await();
}


async fn asynchronous_task () -> uszie {
    println!{"hello world"};                    // work
    let n_char = read_from_disk().await();      // wait ...
    return n_char                               // work
}
//  is equivalent with 
fn asynchronous_task () -> impl Future<Output = usize> {
    // 
}

// nested async task
async fn nested_async_task () -> usize {
    println!{"call and wait till foo() is complete"};
    foo.await()

    println!{"call and wait till bar() is complete"};
    bar.await()
}
```


![state_machine](images/state_machine.drawio.svg "state_machine")



### Future


### The big picture
