//! This is a simple example introducing the core concepts of the Wasmer API.
//!
//! You can run the example directly by executing the following in the Wasmer root:
//!
//! ```shell
//! cargo run --example hello-wasmer --release --features "cranelift"
//! ```

use wasmer::{imports, Function, Instance, Module, Store, TypedFunction};

fn main() -> anyhow::Result<()> {
    let wasm_bytes = include_bytes!("../wasm_files/hello_wasmer.wasm");

    // Create a Store.
    let mut store = Store::default();

    // We then use our store and Wasm bytes to compile a `Module`.
    // A `Module` is a compiled WebAssembly module that isn't ready to execute yet.
    let module = Module::new(&store, wasm_bytes)?;

    // We define a function to act as our "env" "hello_wasmer" function imported in the
    // Wasm program above.
    fn say_hello_wasmer() {
        println!("Hello, wasmer!")
    }

    // We then create an import object so that the `Module`'s imports can be satisfied.
    let import_object = imports! {
        // We use the default namespace "env".
        "env" => {
            // And call our function "hello_wasmer".
            "hello_wasmer" => Function::new_typed(&mut store, say_hello_wasmer),
        }
    };

    // We then use the `Module` and the import object to create an `Instance`.
    //
    // An `Instance` is a compiled WebAssembly module that has been set up
    // and is ready to execute.
    let instance = Instance::new(&mut store, &module, &import_object)?;

    // We get the `TypedFunction` with no parameters and no results from the instance.
    //
    // Recall that the Wasm module exported a function named "execute", this is getting
    // that exported function from the `Instance`.
    let execute_func: TypedFunction<(), ()> = instance.exports.get_typed_function(&mut store, "execute")?;

    // Finally, we call our exported Wasm function which will call our "hello_wasmer"
    // function and return.
    execute_func.call(&mut store)?;

    Ok(())
}

#[test]
fn test_hello_world() -> anyhow::Result<()> {
    main()
}