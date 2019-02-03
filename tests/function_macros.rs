#[macro_use] extern crate luajit;

use luajit::{State, c_int, ThreadStatus};

fn return_42(state: &mut State) -> c_int {
    state.push(42);

    1
}

#[test]
fn test_lua_fn() {
    let mut state = State::new();
    state.open_libs();

    state.register("return_42", lua_fn!(return_42).unwrap());
    let status = state.do_string("if return_42() ~= 42 then error() end");
    assert_eq!(status, ThreadStatus::Ok);
}

#[test]
fn test_push_lua_function() {
    let mut state = State::new();
    state.open_libs();

    state.push(lua_fn!(return_42));
    state.set_global("return_42");
    let status = state.do_string("if return_42() ~= 42 then error() end");
    assert_eq!(status, ThreadStatus::Ok);
}
