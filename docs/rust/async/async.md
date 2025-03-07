# Async Await


### Overview


![overview](images/overview.drawio.svg "overview")


* To get a better understand about async-await, we should get to understand about concurrency programming.


### Concurrency vs Paralellism


* Paralellism & Concurrency is two approachs to handle multiple tasks at the same time.


* Paralellism:
    + Each thread handle 1 task.
    + Use multiple threads to handle multiple tasks.

* Concurrency:
    + Each thread can handle multiple tasks ->> no need much thread.
    + At a point of time, one thread can execute only 1 task, but the thread can switch between it's tasks.


![concurency_vs_paralellism](images/concurency_vs_paralellism.drawio.svg "concurency_vs_paralellism")


* Comments & Questions:
    + In both approachs, one thread can execute 1 task at a time. Why use Concurrency over Paralellism ?
    + In the image, it's seem Paralellism can finish the Green Task faster than Concurrency ?
    + Paralellism can isolated task. It looks much better.
    + How Concurrency knows how to mix the task ?



* The answers for those questions above is inside the task.
    Concurrency will show it's advantage when work with the tasks which have to wait for events from I/O.
    The CPU core runs much faster than I/O devices. So, for e.g, it has to wait for I/O drivers to write something to disk, or wait for I/O socket, ...etc.


![tasks](images/tasks.drawio.svg "tasks")


* Idle CPU core is a waste of resource.
    + In Paralellism, thread must yeild the CPU to another thread, and let the kernel schedule to have chance to run again.
        Ofcouse, the next thread might belong to another program.
    + In Concurrency, NO yeilding happends. Thread will switch to another task, if current task are waiting for I/O.


![concurency_vs_paralellism_in_deep_](images/concurency_vs_paralellism_in_deep.drawio.svg "concurency_vs_paralellism_in_deep_")


* The image shows that the Green Task might be completed earlier, compare to Paralellism.

* But, Paralellism is easier to implement, because Kernel helps to take care all the threads.

* To implement Concurrency, we need to implement mechanism for thread to be able to hold tasks and switch between tasks.
It's time for Runtime to comes as a rescuer.


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



### Async Await


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
```


### Nested Async Await


Async functions inside another async function.


```rust
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


