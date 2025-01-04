
### 1. Closure

Think of closure as lambdas in C++, or arrow function in JavaScript.

Rust uses struct to present closure.

* Imp Fn trait
```rust
let color = String::from("green");
let print = || println!("color: {}", color);  
// print is a closure
```

```rust
// desugar syntactic:
struct PrintClosure {
    color: & String,
    // borrow **immutable** ownership of the real color value
}
impl Fn for PrintClosure {
    fn call(&self) {
        println!("color: {}", self.color);  
    }
}
```

![fn_closure.drawio.svg](images/fn_closure.drawio.svg "Memory map of closure as Fn Trait Object")


* Imp FnMut trait
```rust
let mut count = 0;
let mut inc = || {      // notice: inc is mutable closure
    count += 1;
    println!("count: {}", count);
};
```

```rust
// desugar syntactic:
struct IncClosure {
    count: &mut i32,
    // borrow **mutable** ownership of the count value
}
impl FnMut for IncClosure {
    fn call_mut(&self) {
        println!("color: {}", self.color);  
    }
}
```

* Imp FnOnce trait
```rust
let movable = Box::new(3);
let consume = || {
    println!("movable: {:?}", movable);
    mem::drop(movable);
};
```

```rust
// desugar syntactic:
struct ConsumeClosure {
    movable: Box<i32>,
}
impl FnOnce for ConsumeClosure {
    fn call_once(&self) {
        println!("movable: {:?}", self.movable);
        mem::drop(self.movable);
        // "drop" causes this closure can be called only 1 time.
    }
}
```


* Using **move** to forces closure to take ownership.

The **local_string** will outlive the closure returned by create_closure().
```rust
fn create_closure() -> impl Fn() {
    let local_string = String::from("hello");

    || {
        println!("local: ", local_string);
    }
}
```

Using **move** to move the value to inside closure
```rust
fn create_closure() -> impl Fn() {
    let local_string = String::from("hello");

    move || {
        println!("local: ", local_string);
    }
}
```

```rust
// desugar syntactic:
struct AnonymousClosure {
    local_string: String,
    // local_string will be move to AnonymousClosure.local_string
}
```

![move_closure.drawio.svg](images/move_closure.drawio.svg "Move closure")
