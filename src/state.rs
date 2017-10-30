use super::ffi::*;
use std::ffi::{CString, CStr};
use libc::c_int;

use super::types::{LuaValue, LuaFunction};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ThreadStatus {
    Ok = LUA_OK as isize,
    Yield = LUA_YIELD as isize,
    RuntimeError = LUA_ERRRUN as isize,
    SyntaxError = LUA_ERRSYNTAX as isize,
    MemoryError = LUA_ERRMEM as isize,
    MsgHandlerError = LUA_ERRERR as isize,
    Unknown
}

impl From<c_int> for ThreadStatus {
    fn from(i: c_int) -> ThreadStatus {
        match i {
            LUA_OK => ThreadStatus::Ok,
            LUA_YIELD => ThreadStatus::Yield,
            LUA_ERRRUN => ThreadStatus::RuntimeError,
            LUA_ERRSYNTAX => ThreadStatus::SyntaxError,
            LUA_ERRMEM => ThreadStatus::MemoryError,
            LUA_ERRERR => ThreadStatus::MsgHandlerError,
            _ => ThreadStatus::Unknown
        }
    }
}

pub struct State {
    state: *mut lua_State,
    owned: bool
}

impl State {
    /// Calls LUA C API to instantiate a new Lua state.
    pub fn new() -> State {
        unsafe {
            State {
                state: lua_open(),
                owned: true
            }
        }
    }

    /// Wraps an existing Lua state. Suitable for use in function handlers
    /// passed to Lua through the C API.
    pub fn from_ptr(state: *mut lua_State) -> State {
        State {
            state: state,
            owned: false,
        }
    }

    /// Opens the Lua standard library on this state.
    /// 
    /// You can use the other `open_*` methods to fine tune
    /// what library functions should be available to Lua scripts.
    pub fn open_libs(&mut self) {
        unsafe {
            luaL_openlibs(self.state);
        }
    }

    /// Opens the Lua basic library on this state.
    pub fn open_base(&mut self) {
        unsafe {
            luaopen_base(self.state);
        }
    }

    /// Opens the Lua math library on this state.
    pub fn open_math(&mut self) {
        unsafe {
            luaopen_math(self.state);
        }
    }

    /// Opens the Lua string library on this state.
    pub fn open_string(&mut self) {
        unsafe {
            luaopen_string(self.state);
        }
    }

    /// Opens the Lua table library on this state.
    pub fn open_table(&mut self) {
        unsafe {
            luaopen_table(self.state);
        }
    }

    /// Opens the Lua io library on this state.
    pub fn open_io(&mut self) {
        unsafe {
            luaopen_io(self.state);
        }
    }

    /// Opens the Lua os library on this state.
    pub fn open_os(&mut self) {
        unsafe {
            luaopen_os(self.state);
        }
    }

    /// Opens the Lua package library on this state.
    pub fn open_package(&mut self) {
        unsafe {
            luaopen_package(self.state);
        }
    }

    /// Opens the Lua debug library on this state.
    pub fn open_debug(&mut self) {
        unsafe {
            luaopen_debug(self.state);
        }
    }

    /// Opens the Lua bit library on this state.
    pub fn open_bit(&mut self) {
        unsafe {
            luaopen_bit(self.state);
        }
    }

    /// Opens the LuaJIT JIT library on this state.
    pub fn open_jit(&mut self) {
        unsafe {
            luaopen_jit(self.state);
        }
    }

    /// Opens the Lua FFI library on this state.
    pub fn open_ffi(&mut self) {
        unsafe {
            luaopen_ffi(self.state);
        }
    }

    /// Executes an arbitrary string as Lua code.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use luajit::{State, ThreadStatus};
    /// 
    /// let mut state = State::new(); // Create new Lua state
    /// state.open_base(); // Need to open base libraries for `print` to be available
    /// 
    /// let status = state.do_string(r#"print("Hello world!")"#);
    /// assert!(status == ThreadStatus::Ok);
    /// ```
    pub fn do_string(&mut self, s: &str) -> ThreadStatus {
        let cstr = CString::new(s).unwrap();
        unsafe {
            luaL_dostring(self.state, cstr.as_ptr()).into()
        }
    }
    
    /// Registers function `f` as a Lua global named `name`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use luajit::{State, ThreadStatus, c_int};
    /// use luajit::ffi::lua_State;
    /// 
    /// unsafe extern "C" fn hello(L: *mut lua_State) -> c_int {
    ///     println!("Hello world!");
    /// 
    ///     0
    /// }
    /// 
    /// let mut state = State::new();
    /// state.register("hello", hello);
    /// 
    /// let status = state.do_string("hello()");
    /// assert!(status == ThreadStatus::Ok);
    /// ```
    /// 
    /// Using an argument.
    /// 
    /// ```
    /// use luajit::{State, ThreadStatus, c_int};
    /// use luajit::ffi::lua_State;
    /// 
    /// unsafe extern "C" fn hello_name(l: *mut lua_State) -> c_int {
    ///     let mut state = State::from_ptr(l);
    ///     match state.to_str(1) {
    ///         Some(s) => println!("Hello {}", s),
    ///         None => println!("You have no name!"),
    ///     }
    /// 
    ///     0
    /// }
    /// 
    /// let mut state = State::new();
    /// state.register("hello", hello_name);
    /// 
    /// let status = state.do_string(r#"hello("world!")"#);
    /// assert!(status == ThreadStatus::Ok);
    /// ```
    pub fn register(&mut self, name: &str, f: LuaFunction) {
        let name = CString::new(name).unwrap();
        unsafe {
            lua_register(self.state, name.as_ptr(), Some(f));
        }
    }

    /// Test if the value at `idx` on the stack is a number.
    pub fn is_number(&mut self, idx: c_int) -> bool {
        unsafe {
            lua_isnumber(self.state, idx) != 0
        }
    }

    /// Test if the value at `idx` on the stack is a string.
    pub fn is_string(&mut self, idx: c_int) -> bool {
        unsafe {
            lua_isstring(self.state, idx) != 0
        }
    }

    /// Test if the value at `idx` on the stack is a boolean.
    pub fn is_bool(&mut self, idx: c_int) -> bool {
        unsafe {
            lua_isboolean(self.state, idx)
        }
    }

    /// Retrieves a string from the Lua stack.
    pub fn to_str(&mut self, idx: c_int) -> Option<&str> {
        let ptr = unsafe {
            lua_tostring(self.state, idx)
        };

        if ptr.is_null() {
            None
        } else {
            let cstr = unsafe {
                CStr::from_ptr(ptr)
            };

            match cstr.to_str() {
                Ok(s) => Some(s),
                Err(_) => None
            }
        }
    }

    /// Return the value on the stack at `idx` as an integer.
    pub fn to_int(&mut self, idx: c_int) -> Option<i32> {
        if self.is_number(idx) {
            unsafe {
                Some(lua_tointeger(self.state, idx) as i32)
            }
        } else {
            None
        }
    }

    /// Return the value on the stack at `idx` as an integer.
    pub fn to_long(&mut self, idx: c_int) -> Option<i64> {
        if self.is_number(idx) {
            unsafe {
                Some(lua_tointeger(self.state, idx) as i64)
            }
        } else {
            None
        }
    }

    /// Return the value on the stack at `idx` as an bool.
    pub fn to_bool(&mut self, idx: c_int) -> Option<bool> {
        if self.is_bool(idx) {
            unsafe {
                Some(lua_toboolean(self.state, idx) != 0)
            }
        } else {
            None
        }
    }

    /// Return the value on the stack at `idx` as an float.
    pub fn to_float(&mut self, idx: c_int) -> Option<f32> {
        if self.is_number(idx) {
            unsafe {
                Some(lua_tonumber(self.state, idx) as f32)
            }
        } else {
            None
        }
    }

    /// Return the value on the stack at `idx` as an double.
    pub fn to_double(&mut self, idx: c_int) -> Option<f64> {
        if self.is_number(idx) {
            unsafe {
                Some(lua_tonumber(self.state, idx) as f64)
            }
        } else {
            None
        }
    }

    /// Pushes a LuaValue to the lua stack.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use luajit::State;
    /// 
    /// let mut state = State::new();
    /// state.push_value(5);
    /// state.push_value("Hello world!");
    /// ```
    pub fn push_value<T>(&mut self, val: T) where T: LuaValue {
        val.push_val(self.state);
    }
}

impl Drop for State {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                lua_close(self.state);
            }
        }
    }
}