pub fn main() {
    let artifacts = lua_src::Build::new().build(lua_src::Lua51);
    artifacts.print_cargo_metadata();
}
