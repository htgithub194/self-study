# Extractor


* The *Path* extractor can be used as simple like this.


    ```rust
    async fn hello_path(Path(name): Path<String>) -> impl IntoResponse {
    }
    ```

* But, how?

* Answer 1:

    * Extractor actually is a data-struct.

    * *Path(name)* is a destructure. Destruct a Path to get a String

    * Example:
    ```rust
    struct Foo {
        x: u32,
        y: u32,
    }
    let faa = Foo { x: 1, y: 2 };

    // destruct faa -> x0, y0
    let Foo { x : x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");
    ```

* Answer 2:

    * Extractor must implement either *FromRequestParts* or *FromRequest*

    * When handler function is executed:

    ```rust

        async fn custom_handler(extractor_1, extractor_2, ..., last_extractor) {
            //
            // At this point, Path(name) is init with name=""
            extractor_1.from_request_parts(request);
            // At this point name="path/from/request"

            extractor_2.from_request_parts(request);
            ...
            last_extractor.from_request(request);

            //
            handler_logic();

            //
            (into_respon_tupple).into_response()
        }
        
    ```

    * Reference source code:

        [impl_handler ](https://github.com/tokio-rs/axum/blob/62470bd5039c4a32b4454d0ceafbbca77c0d4874/axum/src/handler/mod.rs#L206 "axum/src/handler/mod.rs")

