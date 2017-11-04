use super::ffi::*;
use std::ffi::{CString, CStr};
use libc::{c_int, c_void};
use std::{mem, ptr};

use super::types::{LuaValue, LuaFunction, LuaObject};

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

    /// Sets the top of the stack to the valid index `idx`
    pub fn settop(&mut self, idx: i32) {
        unsafe {
            lua_settop(self.state, idx);
        }
    }

    /// Pops a value from the top of the stack
    pub fn pop(&mut self, n: i32) {
        unsafe {
            lua_pop(self.state, n);
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

    pub fn check_userdata<T>(&mut self, idx: i32, t: &str) -> Option<*mut T> {
        unsafe {
            let udata = luaL_checkudata(self.state, idx, CString::new(t).unwrap().as_ptr());

            if udata == ptr::null_mut() {
                None
            } else {
                Some(udata as *mut T)
            }
        }
    }

    /// Pops a value of the Lua stack and sets it as a global value
    /// named `name`
    pub fn set_global(&mut self, name: &str) {
        unsafe {
            lua_setglobal(self.state, CString::new(name).unwrap().as_ptr());
        }
    }

    /// Sets the value of `name` on the table `t` pointed
    /// to by `idx` as the value on the top of the stack.
    /// 
    /// Equivalent to `t[name] = v` where `t` is the value at
    /// `idx` and `v` is the value at the top of the stack
    pub fn set_field(&mut self, idx: i32, name: &str) {
        unsafe {
            lua_setfield(self.state, idx, CString::new(name).unwrap().as_ptr());
        }
    }

    /// Registers all functions in `fns` on the global table `name`. If name
    /// is `None`, all functions are instead registered on the value on the top
    /// of the stack.
    pub fn register_fns(&mut self, name: Option<&str>, fns: Vec<luaL_Reg>) {
        match name {
            Some(s) => unsafe {
                luaL_register(self.state, CString::new(s).unwrap().as_ptr(), fns.as_ptr());
            },
            None => unsafe {
                luaL_register(self.state, ptr::null(), fns.as_ptr());
            }
        }
    }

    /// Copys the value at `idx` to the top of the stack
    pub fn push_value(&mut self, idx: i32) {
        unsafe {
            lua_pushvalue(self.state, idx);
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
    /// state.push(5);
    /// state.push("Hello world!");
    /// ```
    pub fn push<T>(&mut self, val: T) where T: LuaValue {
        val.push_val(self.state);
    }

    /// Creates a new table and pushes it to the top of the stack
    pub fn new_table(&mut self) {
        unsafe {
            lua_newtable(self.state);
        }
    }

    /// Allocates a new Lua userdata block, and returns the pointer
    /// to it. The returned pointer is owned by the Lua state.
    pub fn new_raw_userdata(&mut self, sz: usize) -> *mut c_void {
        unsafe {
            lua_newuserdata(self.state, sz)
        }
    }

    /// Allocates a new Lua userdata block of size `sizeof(T)` for
    /// use to store Rust objects on the Lua stack. The returned
    /// pointer is owned by the Lua state.
    pub fn new_userdata<T>(&mut self) -> *mut T {
        self.new_raw_userdata(mem::size_of::<T>()) as *mut T
    }

    /// Allocates a userdata object on the Lua stack for storing a rust struct.
    /// This method also sets the userdata object's metatable to the metatable
    /// saved for `struct_type`, it will call `lua_fns` and create a new metatable
    /// to store in the Lua registry if one has not already been created.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use luajit::{State, LuaObject, c_int};
    /// use luajit::ffi::{lua_State, luaL_Reg};
    /// 
    /// struct Point2D {
    ///     x: i32,
    ///     y: i32,
    /// }
    /// 
    /// impl LuaObject for Point2D {
    ///     fn lua_fns() -> Vec<luaL_Reg> {
    ///         vec!(luaL_Reg {
    ///             name: b"add\x00".as_ptr() as *const i8,
    ///             func: Some(Point2D::add),
    ///         })
    ///     }
    /// }
    /// 
    /// impl Point2D {
    ///     extern "C" fn add(state: *mut lua_State) -> c_int {
    ///         let mut state = State::from_ptr(state);
    ///         let point = unsafe { &*(state.check_userdata::<Point2D>(1, "Point2D").unwrap()) };
    /// 
    ///         state.push(point.x + point.y);
    /// 
    ///         1
    ///     }
    /// 
    ///     fn new() -> Point2D {
    ///         Point2D {
    ///             x: 0,
    ///             y: 0,
    ///         }
    ///     }
    /// }
    /// 
    /// let mut state = State::new();
    /// state.open_libs();
    /// unsafe {
    ///     *state.new_struct("Point2D") = Point2D::new();
    /// }
    /// state.set_global("point");
    /// state.do_string(r#"print("point:add()""#);
    /// ```
    pub fn new_struct<T>(&mut self, struct_type: &str) -> *mut T where T: LuaObject {
        let userdata = self.new_userdata(); 

        unsafe {
            if luaL_newmetatable(self.state, CString::new(struct_type).unwrap().as_ptr()) == 1 {
                self.new_table();
                self.register_fns(None, T::lua_fns());

                self.push_value(-1);
                self.set_global(struct_type);

                self.set_field(-2, "__index");
            }

            lua_setmetatable(self.state, -2);
        }

        userdata
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