extern crate luajit;

use luajit::types::LuaObject;
use luajit::{State, ffi, c_int, ThreadStatus};

struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl LuaObject for Point2D {
    fn lua_fns() -> Vec<ffi::luaL_Reg> {
        vec!(ffi::luaL_Reg {
            name: b"add\x00".as_ptr() as *const i8,
            func: Some(Point2D::add)
        })
    }
}

impl Point2D {
    pub extern "C" fn add(l: *mut ffi::lua_State) -> c_int {
        let mut state = State::from_ptr(l);
        let point = unsafe { &*state.check_userdata::<Point2D>(1, "Point2D").unwrap() };

        state.push(point.x + point.y);

        1
    }
}

#[test]
pub fn test_new_struct() {
    let mut state = State::new();
    state.open_libs();

    unsafe {
        *state.new_struct("Point2D") = Point2D {
            x: 1,
            y: 4,
        };
    }

    state.set_global("test");
    
    let res = state.do_string("print(test:add())");
    assert!(res == ThreadStatus::Ok);
}