# FromRef


* Use Trait (or macro) to extracts Sub-State from a big State


![fromref](images/fromref.drawio.svg)



```rust
// trait
pub trait FromRef<T> {
    /// Converts to this type from a reference to the input type.
    fn from_ref(input: &T) -> Self;
}

impl<T> FromRef<T> for T
where
    T: Clone,
{
    fn from_ref(input: &T) -> Self {
        input.clone()
    }
}
```


```rust
// macro
#[derive(FromRef, Clone)]
struct AppState {
    auth_token: AuthToken,
    database_pool: DatabasePool,
    // fields can also be skipped
    #[from_ref(skip)]
    api_token: String,
}
```


```rust
// extractor
async fn handler(State(auth_token): State<AuthToken>) {}

async fn other_handler(State(database_pool): State<DatabasePool>) {}

```