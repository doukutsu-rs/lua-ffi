use libc::{c_uchar, c_int};
use super::lua::lua_State;

pub const LUA_FILEHANDLE: &'static [c_uchar] = b"FILE*\x00";
pub const LUA_COLIBNAME: &'static [c_uchar] = b"coroutine\x00";
pub const LUA_MATHLIBNAME: &'static [c_uchar] = b"math\x00";
pub const LUA_STRLIBNAME: &'static [c_uchar] = b"string\x00";
pub const LUA_TABLIBNAME: &'static [c_uchar] = b"table\x00";
pub const LUA_IOLIBNAME: &'static [c_uchar] = b"io\x00";
pub const LUA_OSLIBNAME: &'static [c_uchar] = b"os\x00";
pub const LUA_LOADLIBNAME: &'static [c_uchar] = b"package\x00";
pub const LUA_DBLIBNAME: &'static [c_uchar] = b"debug\x00";
pub const LUA_UTF8LIBNAME: &'static [c_uchar] = b"utf8\x00";

extern "C" {
    pub fn luaopen_base(L: *mut lua_State) -> c_int;
    pub fn luaopen_coroutine(L: *mut lua_State) -> c_int;
    pub fn luaopen_table(L: *mut lua_State) -> c_int;
    pub fn luaopen_io(L: *mut lua_State) -> c_int;
    pub fn luaopen_os(L: *mut lua_State) -> c_int;
    pub fn luaopen_string(L: *mut lua_State) -> c_int;
    pub fn luaopen_utf8(L: *mut lua_State) -> c_int;
    pub fn luaopen_math(L: *mut lua_State) -> c_int;
    pub fn luaopen_debug(L: *mut lua_State) -> c_int;
    pub fn luaopen_package(L: *mut lua_State) -> c_int;
    pub fn luaL_openlibs(L: *mut lua_State);
}
