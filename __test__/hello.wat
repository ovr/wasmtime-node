(module
  (func (export "test_arg1_i32_ri32") (param $0 i32) (result i32)
    local.get $0
  )
  (func (export "test_arg1_i64_ri64") (param $0 i64) (result i64)
    local.get $0
  )
  (func (export "test_arg1_f64_rf64") (param $0 f64) (result f64)
    local.get $0
  )
  (func (export "answer") (result i32)
     i32.const 42
  )
)