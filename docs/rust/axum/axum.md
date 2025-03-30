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

* Qus: I do not see Service in the above code?

    * Ans: because Axum use Middleware or Handler which eventually be converted to Service


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


    * Refer: [Inventing the Service trait](https://tokio.rs/blog/2021-05-14-inventing-the-service-trait "tokio.rs")

### Middleware


```rust
let routes_all = Router::new()
    .nest("/api", routes_apis)
    .layer(middleware::map_response(logging))
    .layer(middleware::from_fn_with_state(
        app_state.clone(), token,
    ))
    .layer(CookieManagerLayer::new())


pub async fn token(mut req: Request<Body>, next: Next) -> Result<Response> {
    // do smt
    Ok(next.run(req).await)
}


async fn logging(res: Response) -> Response {
    // do smt
    res
}
```


![middleware](images/middleware.drawio.svg)



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


    * Ans 2: suitable func will be automatically imlp Handler trait by the macro
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
    [Converting Handlers into Services](https://docs.rs/axum/latest/axum/handler/trait.Handler.html#converting-handlers-into-services "docs.rs/axum/latest/axum/handler")


* Qus: How extractor work?

    * Ans: 
    [impl_handler ](https://github.com/tokio-rs/axum/blob/62470bd5039c4a32b4454d0ceafbbca77c0d4874/axum/src/handler/mod.rs#L206 "axum/src/handler/mod.rs")


* Qus: How IntoResponse work?

    * Ans:
    [impl_into_response](https://github.com/tokio-rs/axum/blob/62470bd5039c4a32b4454d0ceafbbca77c0d4874/axum-core/src/response/into_response.rs#L396 "axum-core/src/response/into_response.rs#L396")


* Custom Extractor:

    ```rust

    struct ExtData {}

    impl <S: Send + Sync> FromRequestParts<S> for ExtData {

        async fn from_request_parts(parts, _state) -> Result<Self> {
            // extract data from parts & put to Self (ExtData)
        }
    }
    ```

### State


* State is shared mutual data between services (handler/miiddleware)

* E.g: 
    * 2 Requests are handled by 2 Handlers.
    * The 2 Handlers both try to get the entry to Database from State.
    * So, the State is a kind of global data, all services has access to State.


* Add State to Router:

    ```rust
    Struct AppState {
        // ...
    }

    let app_state = AppState {...}

    let routes_all = Router::new()
        .nest("/api", routes_apis)
        .layer(middleware::map_response(logging))
        .layer(middleware::from_fn_with_state(
            app_state.clone(), token,
        ))
        .layer(CookieManagerLayer::new())
        .with_state(app_state)
        ;
    ```

* Add State Extractor to Handler:

    ```rust
    async fn handler_with_state_extractor(
        State(state) : State<AppState>,
    ) -> Result<> {
        // handler logic
    }
    ```


* How state is passed to extractor:
    ```rust
    // get() return MethodRouter
    pub fn get<H, T, S>(handler: H) -> MethodRouter<S, Infallible>

    // MethodRouter call to Route. with State=() ???
    impl<B> Service<Request<B>> for Router<()>

    // Route pass STATE to handler
    pub trait Handler<T, S>

    // Handler pass STATE to Extractor
    pub trait FromRequestParts<S>
    ```

* Qus: Route take State=(). What STATE actually passed to Handler?

    * Ans: 
        * Actually, the concrete type of STATE is inferred from the State Extractor
        * The moment you call the *with_state()*, all Handlers so far are converted to Service right away, with the concrete State type

    ```rust
    pub fn with_state<S2>(self, state: S) -> Router<S2>
    ```
