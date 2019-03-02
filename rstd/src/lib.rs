// Copyright 2017-2018 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

// tag::description[]
//! Lowest-abstraction level for the Substrate runtime: just exports useful primitives from std
//! or core/alloc to be used with any code that depends on the runtime.
// end::description[]

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(core_intrinsics))]
#![cfg_attr(not(feature = "std"), feature(alloc))]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(not(feature = "std"))] {
        pub extern crate alloc;
        /*
        extern "C" {
            fn ext_malloc(size: usize) -> *mut u8;
            fn ext_free(ptr: *mut u8);
        }

        /// Wasm allocator
        pub struct WasmAllocator;

        #[global_allocator]
        static ALLOCATOR: WasmAllocator = WasmAllocator;

        mod __impl {
            use core::alloc::{GlobalAlloc, Layout};

            use super::WasmAllocator;

            unsafe impl GlobalAlloc for WasmAllocator {
                unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
                    super::ext_malloc(layout.size()) as *mut u8
                }

                unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
                    super::ext_free(ptr as *mut u8)
                }
            }
        }
        */

        pub use alloc::boxed;
        pub use alloc::collections;
        pub use alloc::prelude as alloc_prelude;
        pub use alloc::rc;
        pub use alloc::string;
        pub use alloc::vec;

        pub use core::borrow;
        pub use core::cell;
        pub use core::clone;
        pub use core::cmp;
        pub use core::convert;
        pub use core::default;
        pub use core::fmt;
        pub use core::hash;
        pub use core::intrinsics;
        pub use core::iter;
        pub use core::marker;
        pub use core::mem;
        pub use core::num;
        pub use core::ops;
        pub use core::option;
        pub use core::prelude::v1 as core_prelude;
        pub use core::ptr;
        pub use core::slice;
        pub use core::str;
        pub use core::result;
    } else {
        pub use std::*;
    }
}

/// Prelude of common useful imports.
///
/// This should include only things which are in the normal std prelude.
pub mod prelude {
    #[cfg(not(feature = "std"))]
    pub use crate::{alloc_prelude::*, core_prelude::*};
    #[cfg(feature = "std")]
    pub use std::prelude::v1::*;

    // Re-export `vec!` macro here, but not in `std` mode, since
    // std's prelude already brings `vec!` into the scope.
    #[cfg(not(feature = "std"))]
    pub use crate::vec;
}