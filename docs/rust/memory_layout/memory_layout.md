### Struct
```rust
struct User {
    id: char,
    active: u8,
    age: i32,
    ...
}
```
![struct.drawio.svg](images/struct.drawio.svg "Memory layout of struct")

### Tupple

``` rust
let  a : (char, u8 i32) = ('a', 100, 500);
```
![tupple.drawio.svg](images/tupple.drawio.svg "Memory layout of tupple")


### Array


``` rust
let array : [i32, 3] = [5, 10, 15];
```
![array.drawio.svg](images/array.drawio.svg "Memory layout of array")


### Vector


``` rust
let v: Vec<i32> = vec![5, 10, 15];
let sl : &[i32] = &v[0..2];
```
![vector.drawio.svg](images/vector.drawio.svg "Memory layout of vector")


### String

String
``` rust
let hello_String = String::from("Hello")
```
![String.drawio.svg](images/String.drawio.svg "Memory layout of String")


Literal string
``` rust
let literal_str = "Literal String"
let slice = &literal_str[8..]
```
![str.drawio.svg](images/str.drawio.svg "Memory layout of str")

### Enum
``` rust
// C style enum
enum IP{
	v4,
	v6 = 500,
}

// C style union
union MyTag {
    int i;
    double d;
    char s[16];
}

// Rust style enum: tagged union
enum CarTag{
	TagNumber(i32),
	TagString(String),
}
```
![enum.drawio.svg](images/enum.drawio.svg "Memory layout of enum")

* Size of enum IP is the size of the highest value.

* Size of enum CarTag is the sum of sizes of tag, TagString

* Using smartpoint, e.g: Box, to reduce the size of enum

```rust
enum CarTag{
    ...
	TagBox(Box<String>),    // Box is pointer which is fixed size
    ...
}
```
![box_enum.drawio.svg](images/box_enum.drawio.svg "Memory layout of enum with box")

* Rust automatically optimizes Options<T> which contains smart pointer data type (e.g: Box)
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

* In case of optimization Options<T>, the matching pattern works by check if the pointer is NULL or not, corresponding to *None* and *Some*

* Matching pattern with enum is just a comparation of the tag value.

![match_enum_asembly.png](images/match_enum_asembly.png "Asmebly of Matching pattern")


### Box

* Use smart pointer Box<> to store data on Heap

* There a 2 use case:
    * Case 1: Declare a variable which has unknow size at compile time
    ```rust
    trait Vehicle { 
        fn drive(&self);
    }

    
    let t : dyn Vehicle // Wrong: define t as an unsize trat object
    let t : Box<dyn Vehicle>; // Correct: define t as a pointer which point to trai object in heap
    t = Box::new(Car);
    t.drive();
    ```



    * Case 2: Recursive data type.
    ```rust
    // : 
    enum List {
        Cons(i32, List),        // Wrong: size of List is unknow
        Cons(i32, Box<List>),   // Correct: sizeof Box is known
        Nil,
    }
    ```
    ![box_recursive_type.png](images/box_recursive_type.png "Recursive data type")


### Trait behaviors


```rust
// static data types has fixed size -> have copy trait
let a1: int = 10;
let a2: int = a1;


// struct or container data types has unknow size -> have move trait
let s1 = String::from("Hello");
let s2 = s1;    // s1 no longer valid


// call clone() to dupplicate the value if the clone function is avaiable
let s2 = s1.clone();    // s1 and s2 both are valid
```

![copy_move_clone.drawio.svg](images/copy_move_clone.drawio.svg "Some trait's behaviors")


### Trait Object

* Reference of trait type is called Trait Object:
```rust
trait Shape {
    fn area(&self);
}

// trait type has unknow size
let t : Box<dyn Shape>

// trait object has known size
// trait is fat pointer which contains 2 other pointers
let t : &dyn Shape = &rect;

```

![trait_object.drawio.svg](images/trait_object.drawio.svg "Memory map of Trait Object")


### Closure

Rust uses struct to present closure.

```rust
let color = String::from("green");
let print = || println!("color: {}", color);  
// print is a closure
```

![fn_closure.drawio.svg](images/fn_closure.drawio.svg "Memory map of closure as Fn Trait Object")

[Read more on closure ...](../closure/closure.md)



### Dispatching

* Static dispatch: passing struct of data type. Also means the type of instance is known at compile time.

* Dynamic dispatch: passing a trait object (a reference to trait type). The type of object is unknow at compile time, because multiple types can implement a same trait.


```rust

// Point
pub struct Point<T> {
    x: T,
    y: T,
}

// trait
pub trait Shape {
    type T;
    fn area(&self) -> Self::T;
}

// data type
pub struct Rectangle<T> {
    top_left: Point<T>,
    bottom_right: Point<T>,
}

// implement trait for data type
impl<T> Shape for Rectangle<T>
{
    fn area(&self) -> T {
        let width = self.bottom_right.x - self.top_left.x;
        let height = self.top_left.y - self.bottom_right.y;
        return width * height
    }
}


// static dispatch
pub fn static_sum_up_area(a: impl Shape<T = f64>, b: impl Shape<T = f64>) -> (f64, f64) {
    // both a & b are instances of data type
    // compiler knows the area() is Rectangle.area()
    return a.area() + b.area()
}

//dynamic dispatch
pub fn dynamic_sum_up_area(a: &dyn Shape<T = f64>, b: &dyn Shape<T = f64>) -> (f64, f64) {
    // both a & b are trait objects
    // compiler does not know which area() func used here
    // becase area() would be either Rectangle.area() or Triagle.area()
    // area() is only known on runtime, when the value is passed to func
    // function pointer vTable will point to the proper area() in runtime 
    return a.area() + b.area()
}
```


### Reference count

* Using RC to have multiple pointers point to the same value

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
