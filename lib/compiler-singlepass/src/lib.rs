//! A WebAssembly `Compiler` implementation using Singlepass.
//!
//! Singlepass is a super-fast assembly generator that generates
//! assembly code in just one pass. This is useful for different applications
//! including Blockchains and Edge computing where quick compilation
//! times are a must, and JIT bombs should never happen.
//!
//! Compared to Cranelift and LLVM, Singlepass compiles much faster but has worse
//! runtime performance.

#![cfg_attr(not(feature = "std"), no_std)]

mod address_map;
mod codegen_x64;
mod common_decl;
mod compiler;
mod config;
mod emitter_x64;
mod machine;
mod x64_decl;

pub use crate::compiler::SinglepassCompiler;
pub use crate::config::Singlepass;

mod lib {
    #[cfg(feature = "core")]
    pub mod std {
        pub use alloc::{borrow, boxed, format, str, string, sync, vec};
        pub use core::{convert, fmt, ops, iter, cmp, usize, u64, i64, i32, u32};

        pub mod collections {
            pub use alloc::collections::vec_deque::VecDeque;
            pub use alloc::collections::btree_map::BTreeMap;
            pub use hashbrown::*;
        }
    }

    #[cfg(feature = "std")]
    pub mod std {
        pub use std::{
            borrow, boxed, collections, convert, fmt, format, ops, str, string, sync, usize, u64, i64, i32, u32, vec, iter, cmp
        };
    }
}

#[cfg(all(feature = "std", feature = "core"))]
compile_error!(
    "The `std` and `core` features are both enabled, which is an error. Please enable only once."
);

#[cfg(all(not(feature = "std"), not(feature = "core")))]
compile_error!("Both the `std` and `core` features are disabled. Please enable one of them.");

#[cfg(feature = "core")]
extern crate alloc;