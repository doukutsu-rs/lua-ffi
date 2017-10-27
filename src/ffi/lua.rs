#![allow(non_camel_case_types)]
use libc::{c_int, c_uchar, c_schar, c_double, c_void, size_t, ptrdiff_t};
use super::lauxlib::luaL_newstate;

pub const LUA_VERSION: &'static [c_uchar] = b"Lua 5.1\x00";
pub const LUA_RELEASE: &'static [c_uchar] = b"Lua 5.1.4\x00";
pub const LUA_VERSION_NUM: c_int = 501;

pub const LUA_SIGNATURE: &'static [c_uchar] = b"\033Lua\x00";

pub const LUA_MULTIRET: c_int = -1;

pub const LUA_REGISTRYINDEX: c_int = -10000;
pub const LUA_ENVIRONINDEX: c_int = -10001;
pub const LUA_GLOBALSINDEX: c_int = -10002;

#[inline(always)]
pub fn lua_upvalueindex(i: i32) -> c_int {
    LUA_GLOBALSINDEX - i
}

pub const LUA_YIELD: c_int	= 1;
pub const LUA_ERRRUN: c_int = 2;
pub const LUA_ERRSYNTAX: c_int = 3;
pub const LUA_ERRMEM: c_int = 4;
pub const LUA_ERRERR: c_int = 5;

pub type lua_State = c_void;

pub type lua_CFunction = Option<unsafe extern "C" fn(s: *mut lua_State) -> c_int>;

pub type lua_Reader = Option<unsafe extern "C" fn(L: *mut lua_State, ud: *mut c_void, sz: *mut size_t) -> *const c_uchar>;
pub type lua_Writer = Option<unsafe extern "C" fn(L: *mut lua_State, p: *const c_void, sz: size_t, ud: *mut c_void) -> c_int>;
pub type lua_Alloc = Option<unsafe extern "C" fn(ud: *mut c_void, ptr: *mut c_void, osize: size_t, nsize: size_t) -> *mut c_void>;


pub const LUA_TNONE: c_int = -1;

pub const LUA_TNIL: c_int = 0;
pub const LUA_TBOOLEAN: c_int = 1;
pub const LUA_TLIGHTUSERDATA: c_int = 2;
pub const LUA_TNUMBER: c_int = 3;
pub const LUA_TSTRING: c_int = 4;
pub const LUA_TTABLE: c_int = 5;
pub const LUA_TFUNCTION: c_int = 6;
pub const LUA_TUSERDATA: c_int = 7;
pub const LUA_TTHREAD: c_int = 8;

pub const LUA_MINSTACK: c_int = 20;

// These are constant in LuaJIT
pub type lua_Number = c_double;
pub type lua_Integer = ptrdiff_t;
pub const LUA_IDSIZE: size_t = 60;

pub const LUA_GCSTOP: c_int = 0;
pub const LUA_GCRESTART: c_int = 1;
pub const LUA_GCCOLLECT: c_int = 2;
pub const LUA_GCCOUNT: c_int = 3;
pub const LUA_GCCOUNTB: c_int = 4;
pub const LUA_GCSTEP: c_int = 5;
pub const LUA_GCSETPAUSE: c_int = 6;
pub const LUA_GCSETSTEPMUL: c_int = 7;

extern "C" {
    pub fn lua_newstate(f: lua_Alloc, ud: *mut c_void) -> *mut lua_State;
    pub fn lua_close(L: *mut lua_State);
    pub fn lua_newthread(L: *mut lua_State) -> *mut lua_State;
    pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;

    pub fn lua_gettop(L: *mut lua_State) -> c_int;
    pub fn lua_settop(L: *mut lua_State, idx: c_int);
    pub fn lua_pushvalue(L: *mut lua_State, idx: c_int);
    pub fn lua_remove(L: *mut lua_State, idx: c_int);
    pub fn lua_insert(L: *mut lua_State, idx: c_int);
    pub fn lua_replace(L: *mut lua_State, idx: c_int);
    pub fn lua_checkstack(L: *mut lua_State, sz: c_int) -> c_int;

    pub fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int);

    pub fn lua_isnumber(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_isstring(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_iscfunction(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_isuserdata(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_type(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_typename(L: *mut lua_State, idx: c_int) -> c_int;

    pub fn lua_equal(L: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;
    pub fn lua_rawequal(L: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;
    pub fn lua_lessthan(L: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;

    pub fn lua_tonumber(L: *mut lua_State, idx: c_int) -> lua_Number;
    pub fn lua_tointeger(L: *mut lua_State, idx: c_int) -> lua_Integer;
    pub fn lua_toboolean(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_tolstring(L: *mut lua_State, idx: c_int, len: *mut size_t) -> *const c_schar;
    pub fn lua_objlen(L: *mut lua_State, idx: c_int) -> size_t;
    pub fn lua_tocfunction(L: *mut lua_State, idx: c_int) -> lua_CFunction;
    pub fn lua_touserdata(L: *mut lua_State, idx: c_int) -> *mut c_void;
    pub fn lua_tothread(L: *mut lua_State, idx: c_int) -> *mut lua_State;
    pub fn lua_topointer(L: *mut lua_State, idx: c_int) -> *const c_void;

    // Push functions
    pub fn lua_pushnil(L: *mut lua_State);
    pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    pub fn lua_pushlstring(L: *mut lua_State, s: *const c_schar, l: size_t);
    pub fn lua_pushstring(L: *mut lua_State, s: *const c_schar);
    pub fn lua_pushfstring(L: *mut lua_State, fmt: *const c_schar, ...);
    pub fn lua_pushcclosure(L: *mut lua_State, fun: lua_CFunction, n: c_int);
    pub fn lua_pushboolean(L: *mut lua_State, b: c_int);
    pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut c_void);
    pub fn lua_pushthread(L: *mut lua_State) -> c_int;

    // Get functions
    pub fn lua_gettable(L: *mut lua_State, idx: c_int);
    pub fn lua_getfield(L: *mut lua_State, idx: c_int, k: *const c_schar);
    pub fn lua_rawget(L: *mut lua_State, idx: c_int);
    pub fn lua_rawgeti(L: *mut lua_State, idx: c_int, n: c_int);
    pub fn lua_createtable(L: *mut lua_State, narr: c_int, nrec: c_int);
    pub fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut c_void;
    pub fn lua_getmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
    pub fn lua_getfenv(L: *mut lua_State, idx: c_int);

    // Set functions
    pub fn lua_settable(L: *mut lua_State, idx: c_int);
    pub fn lua_setfield(L: *mut lua_State, idx: c_int, k: *const c_schar);
    pub fn lua_rawset(L: *mut lua_State, idx: c_int);
    pub fn lua_rawseti(L: *mut lua_State, idx: c_int, n: c_int);
    pub fn lua_setmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
    pub fn lua_setfenv(L: *mut lua_State, idx: c_int) -> c_int;

    pub fn lua_call(L: *mut lua_State, nargs: c_int, nresults: c_int);
    pub fn lua_pcall(L: *mut lua_State, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int;
    pub fn lua_cpcall(L: *mut lua_State, func: lua_CFunction, ud: *mut c_void) -> c_int;
    pub fn lua_load(L: *mut lua_State, reader: lua_Reader, dt: *mut c_void, chunkname: *const c_schar) -> c_int;

    pub fn lua_dump(L: *mut lua_State, writer: lua_Writer, data: *mut c_void) -> c_int;

    pub fn lua_yield(L: *mut lua_State, nresults: c_int) -> c_int;
    pub fn lua_resume(L: *mut lua_State, narg: c_int) -> c_int;
    pub fn lua_states(L: *mut lua_State) -> c_int;

    pub fn lua_gc(L: *mut lua_State, what: c_int, data: c_int) -> c_int;

    pub fn lua_error(L: *mut lua_State) -> c_int;
    pub fn lua_next(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_concat(L: *mut lua_State, n: c_int);

    pub fn lua_getallocf(L: *mut lua_State, ud: *mut *mut c_void) -> lua_Alloc;
    pub fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut c_void);
}

#[inline(always)]
pub unsafe fn lua_pop(state: *mut lua_State, n: c_int) {
    lua_settop(state, -n - 1);
}

#[inline(always)]
pub unsafe fn lua_newtable(state: *mut lua_State) {
    lua_createtable(state, 0, 0);
}

#[inline(always)]
pub unsafe fn lua_register(state: *mut lua_State, n: *const c_schar, f: lua_CFunction) {
    lua_pushcfunction(state, f);
    lua_setglobal(state, n);
}

#[inline(always)]
pub unsafe fn lua_pushcfunction(state: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(state, f, 0);
}

#[inline(always)]
pub unsafe fn lua_strlen(state: *mut lua_State, i: c_int) -> size_t {
    lua_objlen(state, i)
}

#[inline(always)]
pub unsafe fn lua_isfunction(state: *mut lua_State, n: c_int) -> bool {
    lua_type(state, n) == LUA_TFUNCTION
}

#[inline(always)]
pub unsafe fn lua_istable(state: *mut lua_State, n: c_int) -> bool {
    lua_type(state, n) == LUA_TTABLE
}

#[inline(always)]
pub unsafe fn lua_islightuserdata(state: *mut lua_State, n: c_int) -> bool {
    lua_type(state, n) == LUA_TLIGHTUSERDATA
}

#[inline(always)]
pub unsafe fn lua_isnil(state: *mut lua_State, n: c_int) -> bool {
    lua_type(state, n) == LUA_TNIL
}

#[inline(always)]
pub unsafe fn lua_isboolean(state: *mut lua_State, n: c_int) -> bool {
    lua_type(state, n) == LUA_TBOOLEAN
}

#[inline(always)]
pub unsafe fn lua_isthread(state: *mut lua_State, n: c_int) -> bool {
    lua_type(state, n) == LUA_TTHREAD
}

#[inline(always)]
pub unsafe fn lua_isnone(state: *mut lua_State, n: c_int) -> bool {
    lua_type(state, n) == LUA_TNONE
}

#[inline(always)]
pub unsafe fn lua_isnoneornil(state: *mut lua_State, n: c_int) -> bool {
    lua_type(state, n) <= 0
}

#[inline(always)]
pub unsafe fn lua_pushliteral(state: *mut lua_State, s: &'static str) {
    use std::ffi::CString;
    let c_str = CString::new(s).unwrap();
    lua_pushlstring(state, c_str.as_ptr(), s.as_bytes().len());
}

#[inline(always)]
pub unsafe fn lua_setglobal(state: *mut lua_State, s: *const c_schar) {
    lua_setfield(state, LUA_GLOBALSINDEX, s);
}

#[inline(always)]
pub unsafe fn lua_getglobal(state: *mut lua_State, s: *const c_schar) {
    lua_getfield(state, LUA_GLOBALSINDEX, s);
}

#[inline(always)]
pub unsafe fn lua_tostring(state: *mut lua_State, i: c_int) {
    lua_tolstring(state, i, 0 as *mut size_t);
}

#[inline(always)]
pub unsafe fn lua_open() -> *mut lua_State {
    luaL_newstate()
}

#[inline(always)]
pub unsafe fn lua_getregistry(state: *mut lua_State) {
    lua_pushvalue(state, LUA_REGISTRYINDEX);
}

#[inline(always)]
pub unsafe fn lua_getgccount(state: *mut lua_State) -> c_int {
    lua_gc(state, LUA_GCCOUNT, 0)
}

pub type lua_Hook = Option<extern "C" fn(L: *mut lua_State, ar: *mut lua_Debug)>;

pub const LUA_HOOKCALL: c_int = 0;
pub const LUA_HOOKRET: c_int = 1;
pub const LUA_HOOKLINE: c_int = 2;
pub const LUA_HOOKCOUNT: c_int = 3;
pub const LUA_HOOKTAILRET: c_int = 4;

pub const LUA_MASKCALL: c_int = (1 << LUA_HOOKCALL);
pub const LUA_MASKRET: c_int = (1 << LUA_HOOKRET);
pub const LUA_MASKLINE: c_int = (1 << LUA_HOOKLINE);
pub const LUA_MASKCOUNT: c_int = (1 << LUA_HOOKCOUNT);

extern "C" {
    pub fn lua_setlevel(from: *mut lua_State, to: *mut lua_State);
    pub fn lua_getstack(L: *mut lua_State, level: c_int, ar: *mut lua_Debug) -> c_int;
    pub fn lua_getinfo(L: *mut lua_State, what: *const c_schar, ar: *mut lua_Debug) -> c_int;
    pub fn lua_getlocal(L: *mut lua_State, ar: *const lua_Debug, n: c_int) -> *const c_schar;
    pub fn lua_setlocal(L: *mut lua_State, ar: *const lua_Debug, n: c_int) -> *const c_schar;
    pub fn lua_getupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) -> *const c_schar;
    pub fn lua_setupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) -> *const c_schar;
    pub fn lua_sethook(L: *mut lua_State, func: lua_Hook, mask: c_int, count: c_int) -> c_int;
    pub fn lua_gethook(L: *mut lua_State) -> lua_Hook;
    pub fn lua_gethookmask(L: *mut lua_State) -> c_int;
    pub fn lua_gethookcount(L: *mut lua_State) -> c_int;
    pub fn lua_upvalueid(L: *mut lua_State, idx: c_int, n: c_int) -> *mut c_void;
    pub fn lua_upvaluejoin(L: *mut lua_State, idx1: c_int, n1: c_int, idx2: c_int, n2: c_int);
    pub fn lua_loadx(L: *mut lua_State, reader: lua_Reader, dt: *mut c_void, chunkname: *const c_schar, mode: *const c_schar) -> c_int;
}

#[repr(C)]
pub struct lua_Debug {
    pub event: c_int,
    pub name: *const c_schar,
    pub namewhat: *const c_schar,
    pub what: *const c_schar,
    pub source: *const c_schar,
    pub currentline: c_int,
    pub nups: c_int,
    pub linedefined: c_int,
    pub lastlinedefined: c_int,
    pub short_src: [c_schar; LUA_IDSIZE],
    i_ci: c_int,
}