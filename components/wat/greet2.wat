(component
    (core module $m
        (memory $memory (export "mem")
            (data "\08\00\00\00" "\15\00\00\00" "Hello from WAT!")
        )
        (func $m-greet (result i32)
            i32.const 0
        )
        (export "greet" (func $m-greet))
    )
    (core instance $M (instantiate $m))
    (alias core export $M "mem" (core memory $c-mem))
    (func $c-greet (result string)
        (canon lift (core func $M "greet")
            string-encoding=utf8
            (memory $c-mem)
        )
    )
    (export "greet" (func $c-greet))
)
