# wasm-bindgen-test

`cargo +nightly test --target wasm32-unknown-unknown`:

```
panicked at 'assertion failed: false', src/lib.rs:10:5
wasm://wasm/001946fa:840



RuntimeError: unreachable
    at std::panicking::update_panic_count::h80a2dab08a552cee (wasm-function[839]:33)
    at core::result::unwrap_failed::haf9867869a21c971 (wasm-function[834]:30)
    at core::ptr::drop_in_place::h725cef5c01ce1c6b (wasm-function[829]:444)
    at core::num::NonZeroUsize::new_unchecked::h75d12f3b7b44178a (wasm-function[22]:163)
    at std::sync::once::Once::call_once::ha3b142b2794c9c51 (wasm-function[76]:57)
    at _$LT$core..option..Option$LT$T$GT$$GT$::take::hdc2062c76ef66df3 (wasm-function[57]:29)
    at core::ops::function::FnMut::call_mut::h6a343f5c1edeb441 (wasm-function[7]:28)
    at core::ptr::drop_in_place::h7fbc3d56a6ab0a70 (wasm-function[104]:3)
    at core::ptr::drop_in_place::h6a00e46419a90e24 (wasm-function[103]:6)
    at std::collections::hash::table::make_hash::h6d0b735e4e76af28 (wasm-function[302]:14)
error: test failed, to rerun pass '--lib'
```
