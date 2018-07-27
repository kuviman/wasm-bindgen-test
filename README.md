# wasm-bindgen-test

`cargo +nightly test --target wasm32-unknown-unknown`:

```
Compiling wasm v0.1.0 (file:///home/kuviman/Temp/wasm)                                                                                                                                                                                
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running target/wasm32-unknown-unknown/debug/deps/wasm-2e4b1daed77c845b.wasm
panicked at 'assertion failed: false', src/main.rs:10:5
wasm://wasm/001946fa:840

^

RuntimeError: unreachable
    at std::panicking::update_panic_count::h80a2dab08a552cee (wasm-function[839]:33)
    at core::result::unwrap_failed::haf9867869a21c971 (wasm-function[834]:30)
    at core::ptr::drop_in_place::h725cef5c01ce1c6b (wasm-function[829]:444)
    at core::num::NonZeroUsize::new_unchecked::h26fa6a5f0c8735e0 (wasm-function[22]:163)
    at std::sync::once::Once::call_once::h09a481c2d8286bb2 (wasm-function[76]:57)
    at _$LT$core..option..Option$LT$T$GT$$GT$::take::hb3344f9217a4f189 (wasm-function[57]:29)
    at alloc::alloc::alloc::h96180252bc2d4e46 (wasm-function[5]:28)
    at core::ptr::drop_in_place::h7fbc3d56a6ab0a70 (wasm-function[104]:3)
    at core::ptr::drop_in_place::h6a00e46419a90e24 (wasm-function[103]:6)
    at std::collections::hash::table::make_hash::h6d0b735e4e76af28 (wasm-function[302]:14)
error: test failed, to rerun pass '--bin wasm'
```
