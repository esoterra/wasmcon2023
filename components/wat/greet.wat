(component
  (core module $m
    (type $m-greet-type (func (result i32)))
    (memory $memory 1)
    (func $m-greet (type $m-greet-type) (result i32)
      i32.const 0
    )
    (export "greet" (func $m-greet))
    (data (memory $memory) (offset i32.const 0) "\08\00\00\00") ;; little-endian for 8
    (data (memory $memory) (offset i32.const 4) "\15\00\00\00") ;; little endian for 15
    (data (memory $memory) (offset i32.const 8) "Hello from WAT!")
  )
  (core instance $M (instantiate $m))
  (type $c-greet-type (func (result string)))
  (func $c-greet (type $c-greet-type)
    (canon lift (core func $M "greet")
        string-encoding=utf8
        (memory 0)
    )
  )
  (export "greet" (func $c-greet))
)