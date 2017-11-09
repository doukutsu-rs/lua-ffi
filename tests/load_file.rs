#[macro_use] extern crate luajit;

use luajit::{State, LuaObject};
use luajit::ffi::luaL_Reg;
use std::path::Path;

struct Point2D {
    x: i32,
    y: i32,
}

impl LuaObject for Point2D {
    fn name() -> *const i8 {
        c_str!("Point2D")
    }

    fn lua_fns() -> Vec<luaL_Reg> {
        vec!(
            lua_func!("new", Point2D::new),
            lua_method!("add", Point2D, Point2D::add),
            lua_method!("setX", Point2D, Point2D::set_x),
            lua_method!("setY", Point2D, Point2D::set_y),
        )
    }
}

impl Point2D {
    fn new(state: &mut State) -> i32 {
        state.push(Point2D {
            x: 0,
            y: 0,
        });
        
        1
    }

    fn add(&mut self, state: &mut State) -> i32 {
        state.push(self.x + self.y);

        1
    }

    fn set_x(&mut self, state: &mut State) -> i32 {
        self.x = state.to_int(2).unwrap();

        0
    }

    fn set_y(&mut self, state: &mut State) -> i32 {
        self.y = state.to_int(2).unwrap();

        0
    }
}

#[test]
fn load_file() {
    let mut state = State::new();
    state.open_libs();
    state.register_struct::<Point2D>();
    state.load_file(Path::new("./tests/lua/test1.lua")).unwrap();
    
    let res = state.pcall(0, 0, 0);
    assert_eq!(res, Ok(()));
}

#[test]
fn do_file() {
    let mut state = State::new();
    state.open_libs();
    state.register_struct::<Point2D>();
    let res = state.do_file(Path::new("./tests/lua/test1.lua"));
    
    assert_eq!(res, Ok(()));
}