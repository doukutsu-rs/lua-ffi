use libc::{c_int, c_schar, size_t};
use super::lua::{LUA_ERRERR, lua_CFunction, lua_State, lua_Number, lua_Integer};

pub const LUA_ERRFILE: c_int = LUA_ERRERR + 1;

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

    pub fn luaL_newstate() -> *mut lua_State;
}