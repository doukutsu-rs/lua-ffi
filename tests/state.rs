extern crate luajit;

use luajit::{State, ThreadStatus};

#[test]
fn do_valid_string() {
    let mut state = State::new();
    state.open_libs();
    
    let status = state.do_string(r#"print("Hello world!")"#);
    assert!(status == ThreadStatus::Ok);
}

#[test]
fn do_invalid_string() {
    let mut state = State::new();
    state.open_libs();

    let status = state.do_string("aqdw98hdqw");
    assert!(status == ThreadStatus::SyntaxError);
}