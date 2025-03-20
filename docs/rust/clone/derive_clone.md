# \#[Derive Clone]


### What the macro does

* Automatically implement Clone trait for a type


### Implementation for normal type


* What we write:
```rust
#[derive(Clone)]
struct Foo {
    a: u32
}

```

* can be translated to:

```rust
impl Clone for Foo {
  fn clone(&self) -> Self {
    Foo {
      a: self.a.clone(),
    }
  }
}
```

### Implementation for Generric type


* What we write:

```rust
#[derive(Clone)]
struct Foo <T> {
    a: Rc<T>
}

```

* can be translated to:

```rust
impl<T: ::core::clone::Clone> ::core::clone::Clone for Foo<T> {
  #[inline]
  fn clone(&self) -> Foo<B, C> {
    match *self {
      Foo {
        a: ref __self_0_0,
      } => Foo {
        a: ::core::clone::Clone::clone(&(*__self_0_0)),
      },
    }
  }
}
```


* Thing to notice for generic type is that:
    * event *RC\<T>* can guarantee to be clonable for any type *T*
    * but, the type *T* should also implement Clone too


```rust
impl<T: ::core::clone::Clone> ::core::clone::Clone for Foo<T>
```

