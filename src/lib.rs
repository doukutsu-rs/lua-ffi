extern crate libc;

pub mod ffi;
pub mod state;
pub mod types;

pub use state::{State, ThreadStatus};
pub use types::{LuaFunction, LuaObject};

pub use libc::c_int;

/// This macro can be used to automatically generate a `luaL_Reg`
/// struct for the provided function, with name `name`
#[macro_export]
macro_rules! lua_func {
    ($name:expr, $method:path) => {
        {
            #[allow(unused)]
            use luajit;
            unsafe extern "C" fn trampoline(l: *mut luajit::ffi::lua_State) -> luajit::c_int {
                $method(&mut State::from_ptr(l))
            };

            luajit::ffi::lauxlib::luaL_Reg {
                name: c_str!($name),
                func: Some(trampoline),
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
            use luajit;
            unsafe extern "C" fn trampoline(l: *mut luajit::ffi::lua_State) -> luajit::c_int {
                let mut state = luajit::State::from_ptr(l);
                let st = &mut *state.check_userdata::<$st>(1, stringify!($st)).unwrap();

                $method(st, &mut state)
            };

            luajit::ffi::lauxlib::luaL_Reg {
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