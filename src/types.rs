use super::ffi;
use libc::{ptrdiff_t, c_int};
use std::ffi::CString;

pub trait LuaValue {
    fn push_val(self, l: *mut ffi::lua_State);
}

impl LuaValue for i32 {
    fn push_val(self, l: *mut ffi::lua_State) {
        unsafe {
            ffi::lua_pushinteger(l, self as ptrdiff_t)
        }
    }
}

impl LuaValue for i64 {
    fn push_val(self, l: *mut ffi::lua_State) {
        unsafe {
            ffi::lua_pushinteger(l, self as ptrdiff_t)
        }
    }
}

impl LuaValue for u32 {
    fn push_val(self, l: *mut ffi::lua_State) {
        unsafe {
            ffi::lua_pushinteger(l, self as ptrdiff_t)
        }
    }
}

impl LuaValue for u64 {
    fn push_val(self, l: *mut ffi::lua_State) {
        unsafe {
            ffi::lua_pushinteger(l, self as ptrdiff_t)
        }
    }
}

impl <'a> LuaValue for &'a str {
    fn push_val(self, l: *mut ffi::lua_State) {
        let cstr = CString::new(self).unwrap();
        unsafe {
            ffi::lua_pushstring(l, cstr.as_ptr());
        }
    }
}

impl LuaValue for bool {
    fn push_val(self, l: *mut ffi::lua_State) {
        unsafe {
            if self {
                ffi::lua_pushboolean(l, 1);
            } else {
                ffi::lua_pushboolean(l, 0);
            }
        }
    }
}

impl LuaValue for f32 {
    fn push_val(self, l: *mut ffi::lua_State) {
        unsafe {
            ffi::lua_pushnumber(l, self as f64)
        }
    }
}

impl LuaValue for f64 {
    fn push_val(self, l: *mut ffi::lua_State) {
        unsafe {
            ffi::lua_pushnumber(l, self)
        }
    }
}

impl LuaValue for LuaFunction {
    fn push_val(self, l: *mut ffi::lua_State) {
        unsafe {
            ffi::lua_pushcfunction(l, Some(self));
        }
    }
}

pub type LuaFunction = unsafe extern "C" fn(L: *mut ffi::lua_State) -> c_int;

pub trait LuaObject {
    fn lua_fns() -> Vec<ffi::luaL_Reg>;
}