# 💻 Bindgen

> https://github.com/rust-lang/rust-bindgen

## 📝 Principe 

Permet de transformer des .h en interface .rs
Cargo recupère les sources, compile & install le binary  ``/home/foo/.cargo/bin/bindgen``

>apt install llvm-dev libclang-dev clang

>cargo install bindgen

## 🔧 Utilisation

> bindgen --allowlist-function 'dmtx.' --allowlist-type 'Dmtx.' --allowlist-var 'Dmtx.*' dmtx.h -o test.rs --no-layout-tests

On précise a rust de ne pas afficher les erreurs de style de code

```rust
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
```

On obtient finalement un fichier FFI pour rust, chez nous **/src/libdmtx/libdmtx_sys.rs**