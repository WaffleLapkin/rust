#![no_std]
#![allow(internal_features)]
#![feature(prelude_import)]
#![feature(lang_items)]
#![feature(allow_internal_unstable)]
#![feature(prelude_2024)]
#![feature(type_ascription)]
#![feature(cfg_eval)]
#![feature(cfg_accessible)]
#![feature(derive_const)]
#![feature(custom_test_frameworks)]
#![feature(c_variadic)]
#![feature(exclusive_wrapper)]
#![feature(concat_idents)]
#![feature(format_args_nl)]
#![feature(trace_macros)]
#![feature(log_syntax)]
#![feature(alloc_error_handler)]
#![feature(test)]
#![feature(concat_bytes)]

extern crate alloc as alloc_crate;

mod libc_alloc;
mod rt;

// Explicit imports disambiguate modules in favor of `alloc` crate.
// `alloc` is needed to override `alloc` (the crate).
pub use alloc_crate::{alloc, borrow, fmt, slice, str, *};
pub use core::*;

// That's how std is doing it :shrug:
pub mod task {
    pub use alloc_crate::task::*;
    pub use core::task::*;
}

// That's how std is doing it :shrug:
pub mod ffi {
    pub use alloc_crate::ffi::{CString, FromVecWithNulError, IntoStringError, NulError};
    pub use core::ffi::{CStr, FromBytesWithNulError};
    //pub use self::os_str::{OsStr, OsString};
    pub use core::ffi::c_void;
    pub use core::ffi::{
        c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
        c_ulong, c_ulonglong, c_ushort,
    };
    pub use core::ffi::{VaList, VaListImpl};
}

pub mod sync {
    pub use alloc_crate::sync::{Arc, Weak};
    pub use core::sync::atomic;
    pub use core::sync::Exclusive;

    // pub use self::barrier::{Barrier, BarrierWaitResult};
    // pub use self::condvar::{Condvar, WaitTimeoutResult};
    // pub use self::mutex::{Mutex, MutexGuard};
    // #[allow(deprecated)]
    // pub use self::once::{Once, OnceState, ONCE_INIT};
    // pub use self::poison::{LockResult, PoisonError, TryLockError, TryLockResult};
    // pub use self::rwlock::{RwLock, RwLockReadGuard, RwLockWriteGuard};

    // pub use self::lazy_lock::LazyLock;
    // pub use self::once_lock::OnceLock;

    // pub(crate) use self::remutex::{ReentrantMutex, ReentrantMutexGuard};

    // pub mod mpsc;
}

// Prelude won't be importent automatically, but can be used in conjunction with
// `#[prelude_import] use std::prelude::rust_2021::*;`
pub mod prelude {
    pub mod v1 {
        pub use crate::convert::{AsMut, AsRef, From, Into};
        pub use crate::iter::{DoubleEndedIterator, ExactSizeIterator};
        pub use crate::iter::{Extend, IntoIterator, Iterator};
        pub use crate::marker::{Send, Sized, Sync, Unpin};
        pub use crate::mem::drop;
        pub use crate::ops::{Drop, Fn, FnMut, FnOnce};
        pub use crate::option::Option::{self, None, Some};
        pub use crate::result::Result::{self, Err, Ok};

        #[allow(deprecated)]
        pub use core::prelude::v1::{
            assert, cfg, column, compile_error, concat, concat_idents, env, file, format_args,
            format_args_nl, include, include_bytes, include_str, line, log_syntax, module_path,
            option_env, stringify, trace_macros, Clone, Copy, Debug, Default, Eq, Hash, Ord,
            PartialEq, PartialOrd,
        };

        pub use core::prelude::v1::concat_bytes;
        pub use core::prelude::v1::{
            alloc_error_handler, bench, derive, global_allocator, /* test, */ test_case,
        };

        pub use baremetal_macros::test;

        #[allow(deprecated)]
        pub use core::prelude::v1::{RustcDecodable, RustcEncodable};

        pub use core::prelude::v1::derive_const;

        pub use crate::borrow::ToOwned;
        pub use crate::boxed::Box;
        pub use crate::string::{String, ToString};
        pub use crate::vec::Vec;
        pub use core::prelude::v1::cfg_accessible;
        pub use core::prelude::v1::cfg_eval;
        pub use core::prelude::v1::type_ascribe;
    }
    pub mod rust_2015 {
        pub use super::v1::*;
    }
    pub mod rust_2018 {
        pub use super::v1::*;
    }

    pub mod rust_2021 {
        pub use super::v1::{test, *};
        pub use core::prelude::rust_2021::*;
    }

    pub mod rust_2024 {
        pub use super::v1::{test, *};
        pub use core::prelude::rust_2024::*;
    }
}
