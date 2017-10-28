#![allow(non_snake_case)]
use libc::{c_int, c_schar, size_t, c_void};
use super::lua::*;
use std::ptr;

pub const LUA_ERRFILE: c_int = LUA_ERRERR + 1;
pub const LUAL_BUFFERSIZE: size_t = 8192;

#[repr(C)]
pub struct luaL_Reg {
    pub name: *const c_schar,
    pub func: lua_CFunction,
}

extern "C" {
    pub fn luaL_openlib(L: *mut lua_State, libname: *const c_schar, l: *const luaL_Reg, nup: c_int);
    pub fn luaL_register(L: *mut lua_State, libname: *const c_schar, l: *const luaL_Reg);
    pub fn luaL_getmetafield(L: *mut lua_State, obj: c_int, e: *const c_schar);
    pub fn luaL_callmeta(L: *mut lua_State, obj: c_int, e: *const c_schar);
    pub fn luaL_typerror(L: *mut lua_State, narg: c_int, tname: *const c_schar);
    pub fn luaL_argerror(L: *mut lua_State, numArg: c_int, extramsg: *const c_schar);
    
    pub fn luaL_checklstring(L: *mut lua_State, numArg: c_int, l: *mut size_t) -> *const c_schar;
    pub fn luaL_optlstring(L: *mut lua_State, numArg: c_int, def: *const c_schar, l: *mut size_t) -> *const c_schar;
    pub fn luaL_checknumber(L: *mut lua_State, numArg: c_int) -> lua_Number;
    pub fn luaL_optnumber(L: *mut lua_State, nArg: c_int, def: lua_Number) -> lua_Number;
    pub fn luaL_checkinteger(L: *mut lua_State, numArg: c_int) -> lua_Integer;
    pub fn luaL_optinteger(L: *mut lua_State, nArg: c_int, def: lua_Integer) -> lua_Integer;
    
    pub fn luaL_checkstack(L: *mut lua_State, sz: c_int, msg: *const c_schar);
    pub fn luaL_checktype(L: *mut lua_State, narg: c_int, t: c_int);
    pub fn luaL_checkany(L: *mut lua_State, narg: c_int);

    pub fn luaL_newmetatable(L: *mut lua_State, tname: *const c_schar) -> c_int;
    pub fn luaL_checkudata(L: *mut lua_State, ud: c_int, tname: *const c_schar) -> *mut c_void;

    pub fn luaL_where(L: *mut lua_State, lvl: c_int);
    pub fn luaL_error(L: *mut lua_State, fmt: *const c_schar, ...) -> c_int;

    pub fn luaL_checkoption(L: *mut lua_State, narg: c_int, def: *const c_schar, lst: *const *const c_schar) -> c_int;

    pub fn luaL_ref(L: *mut lua_State, t: c_int) -> c_int;
    pub fn luaL_unref(L: *mut lua_State, t: c_int, r: c_int);

    pub fn luaL_loadfile(L: *mut lua_State, filename: *const c_schar) -> c_int;
    pub fn luaL_loadbuffer(L: *mut lua_State, buff: *const c_schar, sz: size_t, name: *const c_schar) -> c_int;
    pub fn luaL_loadstring(L: *mut lua_State, s: *const c_schar) -> c_int;

    pub fn luaL_newstate() -> *mut lua_State;

    pub fn luaL_gsub(L: *mut lua_State, s: *const c_schar, p: *const c_schar, r: *const c_schar) -> *const c_schar;
    pub fn luaL_findtable(L: *mut lua_State, idx: c_int, fname: *const c_schar, szhint: size_t) -> *const c_schar;

    pub fn luaL_fileresult(L: *mut lua_State, stat: c_int, fname: *const c_schar) -> c_int;
    pub fn luaL_execresult(L: *mut lua_State, stat: c_int) -> c_int;
    pub fn luaL_loadfilex(L: *mut lua_State, filename: *const c_schar, mode: *const c_schar) -> c_int;
    pub fn luaL_loadbufferx(L: *mut lua_State, buff: *const c_schar, sz: size_t, name: *const c_schar, mode: *const c_schar) -> c_int;
    pub fn luaL_traceback(L: *mut lua_State, L1: *mut lua_State, msg: *const c_schar, level: c_int);
}

#[inline(always)]
pub unsafe fn luaL_argcheck(L: *mut lua_State, cond: c_int, numArg: c_int, extramsg: *const c_schar) {
    if cond == 0 {
        luaL_argerror(L, numArg, extramsg);
    }
}

#[inline(always)]
pub unsafe fn luaL_checkstring(L: *mut lua_State, n: c_int) -> *const c_schar {
    luaL_checklstring(L, n, ptr::null_mut())
}

#[inline(always)]
pub unsafe fn luaL_optstring(L: *mut lua_State, n: c_int, d: *const c_schar) -> *const c_schar {
    luaL_optlstring(L, n, d, ptr::null_mut())
}

#[inline(always)]
pub unsafe fn luaL_checkint(L: *mut lua_State, n: c_int) -> c_int {
    luaL_checkinteger(L, n) as c_int
}

#[inline(always)]
pub unsafe fn luaL_optint(L: *mut lua_State, narg: c_int, d: c_int) -> c_int {
    luaL_optinteger(L, narg, d as lua_Integer) as c_int
}

#[inline(always)]
pub unsafe fn luaL_typename(L: *mut lua_State, i: c_int) -> c_int {
    lua_typename(L, lua_type(L, i))
}

#[inline(always)]
pub unsafe fn luaL_dofile(L: *mut lua_State, filename: *const c_schar) -> c_int {
    let status = luaL_loadfile(L, filename);
    if status == 0 {
        lua_pcall(L, 0, LUA_MULTIRET, 0)
    } else {
        status
    }
}

#[inline(always)]
pub unsafe fn luaL_dostring(L: *mut lua_State, s: *const c_schar) -> c_int {
    let status = luaL_loadstring(L, s);
    if status == 0 {
        lua_pcall(L, 0, LUA_MULTIRET, 0)
    } else {
        status
    }
}

#[inline(always)]
pub unsafe fn luaL_getmetatable(L: *mut lua_State, n: *const c_schar) {
    lua_getfield(L, LUA_REGISTRYINDEX, n);
}

#[repr(C)]
pub struct luaL_Buffer {
    pub p: *mut c_schar,
    pub lvl: c_int,
    pub L: *mut lua_State,
    pub buffer: [c_schar; LUAL_BUFFERSIZE],
}

extern "C" {
    pub fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    pub fn luaL_prepbuffer(B: *mut luaL_Buffer) -> *mut c_schar;
    pub fn luaL_addlstring(B: *mut luaL_Buffer, s: *const c_schar, l: size_t);
    pub fn luaL_addstring(B: *mut luaL_Buffer, s: *const c_schar);
    pub fn luaL_addvalue(B: *mut luaL_Buffer);
    pub fn luaL_pushresult(B: *mut luaL_Buffer);    
}

pub const LUA_NOREF: c_int = -2;
pub const LUA_REFNIL: c_int = -1;

#[inline(always)]
pub unsafe fn lua_ref(L: *mut lua_State, lock: c_int) -> c_int {
    if lock != 0 {
        luaL_ref(L, LUA_REGISTRYINDEX)
    } else {
        lua_pushstring(L, b"unlocked references are obsolete\x00".as_ptr() as *const c_schar);
        lua_error(L);
        0
    }
}

#[inline(always)]
pub unsafe fn lua_unref(L: *mut lua_State, r: c_int) {
    luaL_unref(L, LUA_REGISTRYINDEX, r);
}

#[inline(always)]
pub unsafe fn lua_getref(L: *mut lua_State, r: c_int) {
    lua_rawgeti(L, LUA_REGISTRYINDEX, r);
}
