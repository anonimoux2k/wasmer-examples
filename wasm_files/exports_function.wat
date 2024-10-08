(module
  (type $sum_t (func (param i32 i32) (result i32)))
  (type $sub_t (func (param i32 i32) (result i32)))

  (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
    local.get $x
    local.get $y
    i32.add)
  (func $sub_f (type $sub_t) (param $x i32) (param $y i32) (result i32)
    local.get $x
    local.get $y
    i32.sub)

  (export "sum" (func $sum_f))
  (export "sub" (func $sub_f)))