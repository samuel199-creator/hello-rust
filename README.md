### How to reproduce

##### Instructions
```bash
$ git clone --recurse-submodules -j8 https://github.com/samuel199-creator/hello-rust.git
$ cd hello-rust
$ docker run -v $(pwd):/root/sgx -ti -d --name test-runner baiduxlab/sgx-rust:1604-1.1.3
$ docker exec -ti test-runner bash
$ cd ~/sgx/enclave/networking/
$ cargo build # build ok
$ cd ~/sgx/enclave/some_algo/
$ cargo build # build ok
$ cd .. # /root/sgx/enclave
$ cargo build # crash here.
```

##### Error Log
```log
   Compiling networking v0.1.0 (/root/sgx/enclave/networking)
warning: unused imports: `Arc`, `SgxMutex`
  --> some_algo/src/lib.rs:13:17
   |
13 | use std::sync::{Arc, SgxMutex};
   |                 ^^^  ^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused `#[macro_use]` import
  --> some_algo/src/lib.rs:20:1
   |
20 | #[macro_use]
   | ^^^^^^^^^^^^

warning: unused import: `anyhow::Result`
  --> some_algo/src/lib.rs:29:5
   |
29 | use anyhow::Result;
   |     ^^^^^^^^^^^^^^

warning: 3 warnings emitted

warning: unused `#[macro_use]` import
 --> networking/src/lib.rs:5:1
  |
5 | #[macro_use]
  | ^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused `#[macro_use]` import
  --> networking/src/lib.rs:13:1
   |
13 | #[macro_use]
   | ^^^^^^^^^^^^

error: duplicate lang item in crate `std` (which `itertools` depends on): `f32_runtime`.
  |
  = note: the lang item is first defined in crate `sgx_tstd` (which `networking` depends on)
  = note: first definition in `sgx_tstd` loaded from /root/sgx/enclave/target/debug/deps/libsgx_tstd-ac0928b7a8a87414.rmeta
  = note: second definition in `std` loaded from /root/.rustup/toolchains/nightly-2020-10-25-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-3010daceac92f8fa.rlib

error: duplicate lang item in crate `std` (which `itertools` depends on): `f64_runtime`.
  |
  = note: the lang item is first defined in crate `sgx_tstd` (which `networking` depends on)
  = note: first definition in `sgx_tstd` loaded from /root/sgx/enclave/target/debug/deps/libsgx_tstd-ac0928b7a8a87414.rmeta
  = note: second definition in `std` loaded from /root/.rustup/toolchains/nightly-2020-10-25-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-3010daceac92f8fa.rlib

error: duplicate lang item in crate `std` (which `itertools` depends on): `panic_impl`.
  |
  = note: the lang item is first defined in crate `sgx_tstd` (which `networking` depends on)
  = note: first definition in `sgx_tstd` loaded from /root/sgx/enclave/target/debug/deps/libsgx_tstd-ac0928b7a8a87414.rmeta
  = note: second definition in `std` loaded from /root/.rustup/toolchains/nightly-2020-10-25-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-3010daceac92f8fa.rlib

error: duplicate lang item in crate `std` (which `itertools` depends on): `begin_panic`.
  |
  = note: the lang item is first defined in crate `sgx_tstd` (which `networking` depends on)
  = note: first definition in `sgx_tstd` loaded from /root/sgx/enclave/target/debug/deps/libsgx_tstd-ac0928b7a8a87414.rmeta
  = note: second definition in `std` loaded from /root/.rustup/toolchains/nightly-2020-10-25-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-3010daceac92f8fa.rlib

error: duplicate lang item in crate `std` (which `itertools` depends on): `oom`.
  |
  = note: the lang item is first defined in crate `sgx_tstd` (which `networking` depends on)
  = note: first definition in `sgx_tstd` loaded from /root/sgx/enclave/target/debug/deps/libsgx_tstd-ac0928b7a8a87414.rmeta
  = note: second definition in `std` loaded from /root/.rustup/toolchains/nightly-2020-10-25-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-3010daceac92f8fa.rlib

error: aborting due to 5 previous errors; 2 warnings emitted

error: could not compile `networking`

To learn more, run the command again with --verbose.
```