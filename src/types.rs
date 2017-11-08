use super::ffi;
use libc::{ptrdiff_t, c_int};
use std::ffi::CString;
use super::State;

/// Represents any value that can be pushed onto the Lua stack
pub trait LuaValue {
    /// `push_val` should push the value of this type the the top
    /// of the stack on Lua state `l`.
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

impl LuaValue for String {
    fn push_val(self, l: *mut ffi::lua_State) {
        let r: &str = self.as_ref();
        let cstr = CString::new(r).unwrap();
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

impl <T> LuaValue for T where T: LuaObject {
    fn push_val(self, l: *mut ffi::lua_State) {
        let mut state = State::from_ptr(l);
        unsafe {
            *state.new_struct::<T>() = self;
        }
    }
}

pub type LuaFunction = unsafe extern "C" fn(L: *mut ffi::lua_State) -> c_int;

/// Structs can implement this trait to enable easy interaction with
/// the Lua stack. Any struct implementing this trait can be pushed
/// to the Lua stack as userdata.
pub trait LuaObject {
    /// The string returned by this method will serve as the name
    /// of this type's metatable in the Lua registry. A good value
    /// is the name of the type LuaObject is being implemented for.
    /// 
    /// The `c_str!` macro can be used to declare C string constants.
    fn name() -> *const i8;

    /// Return a list of all Lua functions on this struct. They will
    /// be registered in the metatable automatically.
    fn lua_fns() -> Vec<ffi::luaL_Reg>;
}