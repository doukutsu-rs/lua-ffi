use super::ffi::*;
use std::ffi::{CString, CStr};
use libc::{c_int, c_void};
use std::{mem, ptr};
use std::path::Path;

use super::types::{LuaValue, LuaFunction, LuaObject};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ThreadStatus {
    Ok = LUA_OK as isize,
    Yield = LUA_YIELD as isize,
    RuntimeError = LUA_ERRRUN as isize,
    SyntaxError = LUA_ERRSYNTAX as isize,
    MemoryError = LUA_ERRMEM as isize,
    MsgHandlerError = LUA_ERRERR as isize,
    FileError = LUA_ERRFILE as isize,
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
            LUA_ERRFILE => ThreadStatus::FileError,
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

    /// Maps to `lua_call`, calls the function on the top of the
    /// stack. 
    pub fn call(&mut self, nargs: i32, nres: i32) {
        unsafe {
            lua_call(self.state, nargs, nres);
        }
    }

    /// Maps to `lua_pcall` and automatically catches an error, returning
    /// the string on the top of the stack as an `Err` result.
    pub fn pcall(&mut self, nargs: i32, nres: i32, err_func: i32) -> Result<(), (ThreadStatus, String)> {
        let res: ThreadStatus = unsafe {
            lua_pcall(self.state, nargs, nres, err_func).into()
        };

        if res != ThreadStatus::Ok {
            Err((res, self.to_str(-1).unwrap_or_default().to_owned()))
        } else {
            Ok(())
        }
    }

    /// Maps directly to `lua_pcall` without additional handling.
    pub fn pcallx(&mut self, nargs: i32, nres: i32, err_func: i32) -> ThreadStatus {
        unsafe {
            lua_pcall(self.state, nargs, nres, err_func).into()
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

    /// Test if the value at `idx` on the stack is a userdata object
    pub fn is_userdata(&mut self, idx: c_int) -> bool {
        unsafe {
            lua_isuserdata(self.state, idx) != 0
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

    /// Returns the userdata on the top of the Lua stack as a raw pointer
    pub fn to_raw_userdata(&mut self, idx: c_int) -> Option<*mut c_void> {
        if self.is_userdata(idx) {
            unsafe {
                Some(lua_touserdata(self.state, idx))
            }
        } else {
            None
        }
    }

    /// Returns the userdata from the top of the Lua stack, cast as a pointer
    /// to type `T`
    /// 
    /// See [`new_userdata`](#method.new_userdata) for more usage.
    pub fn to_userdata<T>(&mut self, idx: c_int) -> Option<*mut T> {
        self.to_raw_userdata(idx).map(|pt| pt as *mut T)
    }

    /// Validates that the userdata at `idx` has metatable `ty` from the Lua registry
    /// and returns a pointer to the userdata object
    pub fn check_userdata_ex<T>(&mut self, idx: c_int, ty: &str) -> Option<*mut T> {
        unsafe {
            let udata = luaL_checkudata(self.state, idx, CString::new(ty).unwrap().as_ptr());

            if udata == ptr::null_mut() {
                None
            } else {
                Some(udata as *mut T)
            }
        }
    }

    /// Validates that the userdata at `idx` is an instance of struct `T` where
    /// `T` implements `LuaObject`, and returns a pointer to the userdata object
    pub fn check_userdata<T>(&mut self, idx: i32) -> Option<*mut T> where T: LuaObject {
        unsafe {
            let udata = luaL_checkudata(self.state, idx, T::name());

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
    pub fn register_fns(&mut self, name: Option<&str>, mut fns: Vec<luaL_Reg>) {
        // Add a sentinel struct, even if one already exists adding a second
        // shouldn't break anything and incur minimal overhead
        fns.push(luaL_Reg {
            name: ptr::null(),
            func: None
        });
        
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
        self.checkstack(1);
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
    /// 
    /// Can also be used with structs that implement `LuaObject`
    /// 
    /// ```
    /// #[macro_use] extern crate luajit;
    /// 
    /// use luajit::{State, LuaObject, c_int};
    /// use luajit::ffi::luaL_Reg;
    /// 
    /// struct Point2D {
    ///     x: i32,
    ///     y: i32,
    /// }
    /// 
    /// impl LuaObject for Point2D {
    ///     fn name() -> *const i8 {
    ///         c_str!("Point2D")
    ///     }
    /// 
    ///     fn lua_fns() -> Vec<luaL_Reg> {
    ///         vec!(lua_method!("add", Point2D, Point2D::add))
    ///     }
    /// }
    /// 
    /// impl Point2D {
    ///     fn add(&mut self, state: &mut State) -> c_int {
    ///         state.push(self.x + self.y);
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
    /// 
    /// fn main() {
    ///     let mut state = State::new();
    ///     state.open_libs();
    ///     state.push(Point2D::new());
    ///     state.set_global("point");
    ///     let res = state.do_string(r#"print(point:add())"#);
    ///     assert_eq!(res, luajit::ThreadStatus::Ok);
    /// }
    /// ```
    pub fn push<T>(&mut self, val: T) where T: LuaValue {
        val.push_val(self.state);
    }
    
    /// Push a new nil value onto the Lua stack.
    pub fn push_nil(&mut self) {
        self.checkstack(1);
        unsafe {
            lua_pushnil(self.state);
        }
    }

    /// Gets a value from the globals object and pushes it to the 
    /// top of the stack.
    pub fn get_global(&mut self, name: &str) {
        self.checkstack(1);
        unsafe {
            lua_getglobal(self.state, CString::new(name).unwrap().as_ptr());
        }
    }

    /// Gets a value `name` from the table on the stack at `idx` and
    /// and pushes the fetched value to the top of the stack.
    pub fn get_field(&mut self, idx: i32, name: &str) {
        self.checkstack(1);
        unsafe {
            lua_getfield(self.state, idx, CString::new(name).unwrap().as_ptr());
        }
    }

    /// Creates a new table and pushes it to the top of the stack
    pub fn new_table(&mut self) {
        self.checkstack(1);
        unsafe {
            lua_newtable(self.state);
        }
    }

    /// Allocates a new Lua userdata block, and returns the pointer
    /// to it. The returned pointer is owned by the Lua state.
    pub fn new_raw_userdata(&mut self, sz: usize) -> *mut c_void {
        self.checkstack(1);
        unsafe {
            let new_ptr = lua_newuserdata(self.state, sz);
            if new_ptr == ptr::null_mut() {
                panic!("Lua returned null pointer allocating new userdata");
            }

            new_ptr
        }
    }

    /// Allocates a new Lua userdata block of size `sizeof(T)` for
    /// use to store Rust objects on the Lua stack. The returned
    /// pointer is owned by the Lua state.
    /// 
    /// # Examples
    /// 
    /// Useful for pushing an arbitrary struct to the Lua stack
    /// 
    /// ```
    /// extern crate luajit;
    /// 
    /// use luajit::State;
    /// 
    /// struct Point2D {
    ///     x: i32,
    ///     y: i32,
    /// }
    /// 
    /// impl Point2D {
    ///     fn add(&self) -> i32 {
    ///         self.x + self.y
    ///     }
    ///     
    ///     fn set_x(&mut self, x: i32) {
    ///         self.x = x;
    ///     }
    /// 
    ///     fn set_y(&mut self, y: i32) {
    ///         self.y = y;
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
    /// 
    /// fn main() {
    ///     let mut state = State::new();
    ///     state.open_libs();
    ///     unsafe {
    ///         *state.new_userdata() = Point2D::new();
    ///     }
    /// 
    ///     let point: &mut Point2D = unsafe { &mut *state.to_userdata(-1).unwrap() };
    ///     
    ///     point.set_x(2);
    ///     point.set_y(4);
    /// 
    ///     assert_eq!(point.add(), 6);
    /// }
    /// ```
    pub fn new_userdata<T>(&mut self) -> *mut T {
        self.new_raw_userdata(mem::size_of::<T>()) as *mut T
    }

    /// Registers all of the methods for LuaObject `T` as a global metatable
    /// with name `struct_type` and leaves it on the top of the stack.
    pub fn register_struct<T>(&mut self) where T: LuaObject {
        unsafe {
            if luaL_newmetatable(self.state, T::name()) == 1 {
                self.new_table();
                self.register_fns(None, T::lua_fns());

                self.push_value(-1);
                lua_setglobal(self.state, T::name());

                self.set_field(-2, "__index");
            }
        }
    }

    /// Allocates a userdata object on the Lua stack for storing a rust struct.
    /// This method also sets the userdata object's metatable to the metatable
    /// saved for `struct_type`, it will call `lua_fns` and create a new metatable
    /// to store in the Lua registry if one has not already been created.
    pub(crate) fn new_struct<T>(&mut self) -> *mut T where T: LuaObject {
        let userdata = self.new_userdata(); 

        unsafe {
            self.register_struct::<T>();

            lua_setmetatable(self.state, -2);
        }

        userdata
    }

    /// Maps to `luaL_loadfile`, this method validates that the file exists
    /// before passing it into the Lua C API.
    pub fn load_file(&mut self, path: &Path) -> Result<(), (ThreadStatus, String)> {
        if path.is_file() {
            let p = path.canonicalize().unwrap();
            let full_path = p.to_string_lossy();
            
            unsafe {
                let cstr = CString::new(full_path.as_ref()).unwrap();
                let res: ThreadStatus = luaL_loadfile(self.state, cstr.as_ptr() as *const i8).into();
                if res != ThreadStatus::Ok {
                    Err((res, self.to_str(-1).unwrap_or_default().to_owned()))
                } else {
                    Ok(())
                }
            }
        } else {
            Err((ThreadStatus::FileError, "Path does not exist".to_owned()))
        }
    }

    /// Equivalent of `luaL_dofile`, loads a file and then immediately executes
    /// it with `pcall`, returning the result.
    pub fn do_file(&mut self, path: &Path) -> Result<(), (ThreadStatus, String)> {
        self.load_file(path).and_then(|_| {
            self.pcall(0, LUA_MULTIRET, 0)
        })
    }

    /// Ensures that there are at least `n` free stack slots in the stack. Returns
    /// false if it cannot grow the stack to that size.
    pub fn checkstack(&mut self, n: usize) -> bool {
        unsafe {
            lua_checkstack(self.state, n as c_int) != 0
        }
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