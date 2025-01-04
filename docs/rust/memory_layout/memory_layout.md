### 1. Tupple


``` rust
let  a : (char, u8 i32) = ('a', 100, 500);
```
![tupple.drawio.svg](images/tupple.drawio.svg "Memory layout of tupple")


### 1. Array


``` rust
let array : [i32, 3] = [5, 10, 15];
```
![array.drawio.svg](images/array.drawio.svg "Memory layout of array")


### 1. Vector


``` rust
let v: Vec<i32> = vec![5, 10, 15];
let sl : &[i32] = &v[0..2];
```
![vector.drawio.svg](images/vector.drawio.svg "Memory layout of vector")


### 1. String

String
``` rust
let hello_String = String::from("Hello")
```
![String.drawio.svg](images/String.drawio.svg "Memory layout of String")


Literal string
``` rust
let literal_str = String::from("Literal String")
let slice = &literal_str[8..]
```
![str.drawio.svg](images/str.drawio.svg "Memory layout of str")


### 1. Struct
```rust
struct User {
    id: char,
    active: u8,
    age: i32,
    ...
}
```
![struct.drawio.svg](images/struct.drawio.svg "Memory layout of struct")


### 1. Enum
``` rust
// C style enum
enum IP{
	v4,
	v6 = 500,
}

// Tagged union enum
enum CarTag{
	TagNumber(i32),
	TagString(String),
}
```
![enum.drawio.svg](images/enum.drawio.svg "Memory layout of enum")

* Size of enum IP is the size of the highest value.

* Size of enum CarTag is the sum of sizes of tag, padding, TagString

* Using smartpoint, e.g: Box, to reduce the size of enum
```rust
enum CarTag{
    ...
	TagBox(Box<String>),    // Box is pointer which is fixed size
    ...
}
```
![box_enum.drawio.svg](images/box_enum.drawio.svg "Memory layout of enum with box")


* Rust automatically optimizes Option which contains smart pointer data type (e.g: Box)
```rust
// Option
enum Option<T>{
	None
	Some(T),
}

// Option with smart pointer
let opt: Options<Box<i32>> = Options(Box::new(100)));
```
![option.drawio.svg](images/option.drawio.svg "Memory layout of option")


### 1. Trait behaviors


![copy_move_clone.drawio.svg](images/copy_move_clone.drawio.svg "Some trait's behaviors")



### 1. Reference count

* Using RC to have:

![refcount.drawio.svg](images/refcount.drawio.svg "Memory map of Reference count")

* Data race in multithread:

![rc_datarace.drawio.svg](images/rc_datarace.drawio.svg "Data race")

* Use Atomic Reference Count (Arc) to avoid data race.

    Arc costs small extra performance.

    Both Rc & Arc are immutable. Use mutex to mutate the data.
```rust
let arc_mutext : Arc<Mutex<i32>> = Arc::new(Mutex::new(100))
```

![atomic_rc.drawio.svg](images/atomic_rc.drawio.svg "Atomic Refernce Count")


### 1. Trait Object


![trait_object.drawio.svg](images/trait_object.drawio.svg "Memory map of Trait Object")


### 1. Closure

Think of closure as lambdas in C++, or arrow function in JavaScript.

Rust uses struct to present closure.

```rust
let color = String::from("green");
let print = || println!("color: {}", color);  
// print is a closure
```

![fn_closure.drawio.svg](images/fn_closure.drawio.svg "Memory map of closure as Fn Trait Object")