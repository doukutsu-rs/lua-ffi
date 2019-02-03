//! # LuaJIT RS
//! 
//! `luajit_rs` is a simple wrapper around the LuaJIT project, allowing it to
//! be called from Rust easily and with minimal overhead. Most functions in this
//! crate correspond directly to underlying Lua C API calls
//! 
//! # Examples
//! 
//! ```
//! #[macro_use]
//! extern crate luajit;
//!
//! use luajit::{c_int, State};
//!
//! fn return_42(state: &mut State) -> c_int {
//!     state.push(42);
//!
//!     1
//! }
//!
//! pub fn main() {
//!     let mut state = State::new();
//!     state.open_libs();
//!     state.do_string(r#"print("Hello world!")"#);
//!
//!     state.push(lua_fn!(return_42));
//!     state.set_global("return_42");
//!     state.do_string(r#"print(return_42())"#);
//! }
//! ```

extern crate libc;

pub mod ffi;
pub mod state;
pub mod types;

pub use state::{State, ThreadStatus};
pub use types::{LuaFunction, LuaObject};

pub use libc::c_int;

/// This macro is used to wrap a rust function in an `extern "C"` trampoline
/// to automatically pass a [`State`](state/struct.State.html) struct as the first
/// argument instead of a `lua_State` raw pointer
/// 
/// # Examples
/// 
/// ```
/// #[macro_use] extern crate luajit;
///
/// use luajit::{State, c_int, ThreadStatus};
///
/// fn return_42(state: &mut State) -> c_int {
///     state.push(42);
///
///     1
/// }
///
/// fn main() {
///     let mut state = State::new();
///     state.open_libs();
///
///     state.push(lua_fn!(return_42));
///     state.set_global("return_42");
///     let status = state.do_string("if return_42() ~= 42 then error() end");
///     assert_eq!(status, ThreadStatus::Ok);
///     
///     // Equivalent
///     state.register("return_42", lua_fn!(return_42).unwrap());
/// }
/// ```
#[macro_export]
macro_rules! lua_fn {
    ($method:path) => {
        {
            #[allow(unused)]
            unsafe extern "C" fn trampoline(l: *mut $crate::ffi::lua_State) -> $crate::c_int {
                $method(&mut $crate::State::from_ptr(l))
            };

            Some(trampoline as $crate::LuaFunction)
        }
    }
}

/// This macro can be used to automatically generate a `luaL_Reg`
/// struct for the provided function, with name `name`
#[macro_export]
macro_rules! lua_func {
    ($name:expr, $method:path) => {
        {
            $crate::ffi::lauxlib::luaL_Reg {
                name: c_str!($name),
                func: lua_fn!($method),
            }
        }
    };
}

/// This macro can be used to automatically generate a `luaL_Reg`
/// struct for the provided method, with name `name`. It automatically
/// reads an instances of struct `$st` from userdata and provides it as
/// an argument.
#[macro_export]
macro_rules! lua_method {
    ($name:expr, $st:ty, $method:path) => {
        {
            #[allow(unused)]
            unsafe extern "C" fn trampoline(l: *mut $crate::ffi::lua_State) -> $crate::c_int {
                let mut state = $crate::State::from_ptr(l);
                let st = &mut *state.check_userdata::<$st>(1).unwrap();

                $method(st, &mut state)
            };

            $crate::ffi::lauxlib::luaL_Reg {
                name: c_str!($name),
                func: Some(trampoline),
            }
        }
    }
}

#[macro_export]
macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\x00").as_ptr() as *const i8
    }
}