#[macro_use] extern crate luajit;

use luajit::types::LuaObject;
use luajit::{State, ffi, c_int, ThreadStatus};

struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl LuaObject for Point2D {
    fn name() -> *const i8 {
        c_str!("Point2D")
    }

    fn lua_fns() -> Vec<ffi::luaL_Reg> {
        vec!(
            lua_func!("new", Point2D::new),
            lua_method!("add", Point2D, Point2D::add),
            lua_method!("sub", Point2D, Point2D::sub),
            lua_method!("setX", Point2D, Point2D::set_x),
            lua_method!("setY", Point2D, Point2D::set_y),
        )
    }
}

impl Point2D {
    fn new(state: &mut State) -> c_int {
        state.push(Point2D {
            x: 0,
            y: 0,
        });

        1
    }

    fn add(&mut self, state: &mut State) -> c_int {
        state.push(self.x + self.y);

        1
    }

    fn sub(&mut self, state: &mut State) -> c_int {
        state.push(self.x - self.y);

        1
    }

    fn set_x(&mut self, state: &mut State) -> c_int {
        let new_x = state.to_int(2).unwrap();
        self.x = new_x;

        0
    }

    fn set_y(&mut self, state: &mut State) -> c_int {
        let new_y = state.to_int(2).unwrap();
        self.y = new_y;

        0
    }
}

#[test]
pub fn test_new_struct() {
    let mut state = State::new();
    state.open_libs();

    state.push(Point2D {
        x: 1,
        y: 4,
    });

    state.set_global("test");

    let res = state.do_string("test:setX(4)");
    assert_eq!(res, ThreadStatus::Ok);

    let res = state.do_string("if test:add() ~= 8 then error() end");
    assert_eq!(res, ThreadStatus::Ok);

    let res = state.do_string("if test:sub() ~= 0 then error() end");
    assert_eq!(res, ThreadStatus::Ok);

    let res = state.do_string(
        "foo = Point2D:new()
        foo:setX(2)
        foo:setY(4)
        if foo:add() ~= 6 then error() end"
    );
    assert_eq!(res, ThreadStatus::Ok);
}

struct B {
    pub val: i32
}

impl LuaObject for B {
    fn name() -> *const i8 {
        c_str!("B")
    }

    fn lua_fns() -> Vec<ffi::luaL_Reg> {
        vec![
            lua_method!("add", B, B::add),
            ffi::luaL_Reg {
                name: std::ptr::null(),
                func: None
            }
        ]
    }
}

impl B {
    fn add(&mut self, state: &mut State) -> c_int {
        let x = state.to_int(2).unwrap();
        self.val += x;
        state.push(self.val);

        1
    }
}

#[test]
pub fn test_double_sentinel() {
    let mut state = State::new();
    state.open_libs();

    state.push(B {
        val: 1,
    });

    state.set_global("test");

    let res = state.do_string("test:add(4)");
    assert_eq!(res, ThreadStatus::Ok);

    let res = state.do_string("if test:add(4) ~= 9 then error() end");
    assert_eq!(res, ThreadStatus::Ok);
}