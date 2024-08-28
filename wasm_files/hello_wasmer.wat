(module
  ;; First we define a type with no parameters and no results.
  (type $no_args_no_rets_t (func (param) (result)))

  ;; Then we declare that we want to import a function named "env" "hello_wasmer" with
  ;; that type signature.
  (import "env" "hello_wasmer" (func $hello_wasmer (type $no_args_no_rets_t)))

  ;; Finally we create an entrypoint that calls our imported function.
  (func $execute (type $no_args_no_rets_t)
    (call $hello_wasmer))
  ;; And mark it as an exported function named "execute".
  (export "execute" (func $execute)))