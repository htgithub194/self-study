# Axum


### Architecture overview (just what I know)


* Axum is a Rust crate for building a web server.


* Axum consist of:
    * *hyper* for handle http protocol
    Or, *Tonic* for gRPC
    * *Tokio* as runtime for async tasks
    * *matchit* for matching a request to a Route
    * *tower* for middleware

* Actually, tower plays a very important role in axum.


* Here is the brieft overview of how a request flow inside Axum:

    ![big_infra](images/big_infra.drawio.svg "Infrastructure of Axum")



* To make it more easier to understand:

    ![small_infra](images/small_infra.drawio.svg "Infrastructure of Axum")


```rust
// example from: https://crates.io/crates/axum
#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// handler: root
async fn root() -> &'static str {
    "Hello, World!"
}

// handler: create_user
async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    
    (StatusCode::CREATED, Json(user))

}
```

* Questions:

    * I do not see Service in the above code?

        * Ans: because Axum wraps Service up in form of Middleware or Handlers.


### Tower Service

* Trait *Service*:

    * Turn Request ->> Response

        ```rust
        pub trait Service<Request> {

            type Future: Future<Output = Result<Self::Response, Self::Error>>;

            fn call(&mut self, req: Request) -> Self::Future;
        }
        ```


* Trait *Layer*:

    * To stack up Services

        ```rust
        pub trait Layer<S> {
            type Service;

        fn layer(&self, inner: S) -> Self::Service;
        }
        ```

        ![service_layer](images/service_layer.drawio.svg)


### Handler


* Handler:

    * take extractors to parse Request

    * return sth that can be converted into a Response


* 2 Types of Handler:

    * With STATE

    * Without STATE

![handler](images/handler.drawio.svg)


* Qus: Does Axum accept all func as Handler?

    * Ans 1: handler must be:
        
        * Async func to return future

        * take 0 ->> 16 extractors

        * the request's body part should be extracted at the last extractor 


    * Ans 2: suitale func will be automatically imlp Handler trait by the macro
    [all_the_tuples](https://github.com/tokio-rs/axum/blob/15917c6dbcb4a48707a20e9cfd021992a279a662/axum-core/src/macros.rs#L231 "github link")

        ```rust
        #[rustfmt::skip]
        macro_rules! all_the_tuples {
            ($name:ident) => {
                $name!([], T1);
                $name!([T1], T2);
                $name!([T1, T2], T3);
                $name!([T1, T2, T3], T4);
                $name!([T1, T2, T3, T4], T5);
                $name!([T1, T2, T3, T4, T5], T6);
                $name!([T1, T2, T3, T4, T5, T6], T7);
                $name!([T1, T2, T3, T4, T5, T6, T7], T8);
                $name!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
                $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
                $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
                $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
                $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
                $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13], T14);
                $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14], T15);
                $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15], T16);
            };
        }
        ```


* Qus: Handler is a Service ?

    * Ans: No. But a Handler will be converted into a Service
    [all_the_tuples](https://docs.rs/axum/latest/axum/handler/trait.Handler.html#converting-handlers-into-services "docs.rs/axum/latest/axum/handler")


* Qus: How extractor work?

    * Ans: 
    [impl_handler ](https://github.com/tokio-rs/axum/blob/62470bd5039c4a32b4454d0ceafbbca77c0d4874/axum/src/handler/mod.rs#L206 "axum github repo")


* Custom Extractor:
    TODO

### Middleware

```rust
code
```


![middleware](images/middleware.drawio.svg)



### State


* State Extractor:

    * share mutual data between handler/middleware


```rust
TODO
```


```rust

// S is STATE
pub fn get<H, T, S>(handler: H) -> MethodRouter<S, Infallible>

// Route pass STATE to handler
pub trait Handler<T, S>


// Handler pass STATE to Extractor
pub trait FromRequestParts<S>
```

* STATE is passed from Route -> Extractor.
    * But, at this time, STATE type is generic S type

* Actually, the real type of STATE is inferred from Extractor
