pub fn main() {
    let artifacts = lua_src::Build::new().build(lua_src::Lua53);
    artifacts.print_cargo_metadata();
}
