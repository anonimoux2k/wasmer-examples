(module
  (global $one (export "one") f32 (f32.const 1))
  (global $some (export "some") (mut f32) (f32.const 0))

  (type $get (func (result f32)))
  (func $get_one (type $get) (result f32) (global.get $one))
  (func $get_some (type $get) (result f32) (global.get $some))

  (func (export "set_some") (param f32) (global.set $some (local.get 0)))
  
  (export "get_one" (func $get_one))
  (export "get_some" (func $get_some)))