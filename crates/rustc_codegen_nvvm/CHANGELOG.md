# Changelog

Notable changes to this project will be documented in this file.

## Unreleased

### Dead Code Elimination 

PTX files no longer include useless functions and globals, we have switched to an alternative
method of codegen for the final steps of the codegen. We no longer lazily-load modules using dependency graphs, 
we instead merge all the modules into one then run global DCE on it before giving it to libnvvm.

This means all of the dead code is gone before it gets to the libnvvm stage, drastically lowering the size of 
the built PTX and improving codegen performance. `cuda_std` also has a macro `#[externally_visible]` which can
be used if you want to keep a function around for things like linking multiple PTX files together.

### Libm override

The codegen now has the ability to override [`libm`](https://docs.rs/libm/latest/libm/) functions with 
[`libdevice`](https://docs.nvidia.com/cuda/libdevice-users-guide/introduction.html#introduction) intrinsics.

Libdevice is a bitcode library shipped with every CUDA SDK installation which provides float routines that
are optimized for the GPU and for specific GPU architectures. However, these routines are hard to use automatically because
no_std math crates typically use libm for float things. So users often ended up with needlessly slow or large PTX files
because they used "emulated" routines.

Now, by default (can be disabled in cuda_builder) the codegen will override libm functions with calls to libdevice automatically.
However, if you rely on libm for determinism, you must disable the overriding, since libdevice is not strictly deterministic.
This also makes PTX much smaller generally, in our example path tracer, it slimmed the PTX file from about `3800` LoC to `2300` LoC.

- Trace-level debug is compiled out for release now, decreasing the size of the codegen dll and improving compile times.

## 0.1.1 - 11/26/21

- Fix things using the `bswap` intrinsic panicking.
- (internal) Run clippy and clean things up a bit.