extern crate failure;

use std::fs;
use std::io::Error;
use std::collections::HashMap;
//use failure::Error;

struct Nature {

}

struct Entity {

}

struct Registry {
    natures: HashMap<String, Nature>
}

fn main() {

}

/*

use rlua::{Function, Lua, MetaMethod, UserData, UserDataMethods, Variadic, Value};

struct Entity<'a> {
    name: &'a str,
    description: &'a str,
}

fn main() -> Result<(), Error> {
    let lua = Lua::new();
    let e1 = Entity {name: "table", description: "A boring, brown table."};
    println!("<{}> {}", e1.name, e1.description);

    let fname = "mudlib/a1.lua";
    let code = fs::read_to_string(fname)?;
    lua.context(|lua_ctx|{
       lua_ctx.load(
            &code
       ).exec()?;
        Ok(())
    })?;
    Ok(())
}


fn unmain() -> Result<(), Error> {
    // You can create a new Lua state with `Lua::new()`.  This loads the default Lua std library
    // *without* the debug library.  You can get more control over this with the other
    // `Lua::xxx_new_xxx` functions.
    let lua = Lua::new();

    // In order to interact with Lua values at all, you must do so inside a callback given to the
    // `Lua::context` method.  This provides some extra safety and allows the rlua API to avoid some
    // extra runtime checks.
    lua.context(|lua_ctx| {
        // You can get and set global variables.  Notice that the globals table here is a permanent
        // reference to _G, and it is mutated behind the scenes as Lua code is loaded.  This API is
        // based heavily around sharing and internal mutation (just like Lua itself).

        let globals = lua_ctx.globals();

        globals.set("string_var", "hello")?;
        globals.set("int_var", 42)?;

        Ok(())
    })?;

    lua.context(|lua_ctx| {
        // The Lua state lives inside the top-level `Lua` value, and all state changes persist
        // between `Lua::context` calls.  This is another table reference in another context call,
        // but it refers to the same table _G.

        let globals = lua_ctx.globals();

        assert_eq!(globals.get::<_, String>("string_var")?, "hello");
        assert_eq!(globals.get::<_, i64>("int_var")?, 42);

        Ok(())
    })?;

    lua.context(|lua_ctx| {
        lua_ctx
            .load(
                r#"
                global = 'foo'..'bar'
            "#,
            )
            .set_name("example code")?
            .exec()?;
        Ok(())
    })?;

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        let env_table = lua_ctx.create_table()?;
        let print: Value = globals.get("print")?;
        env_table.set("print", print)?;
        let sv: Value = globals.get("string_varx")?;
        env_table.raw_set("string_var", sv)?;
        lua_ctx.load(r#"
            print(string_var);
            print(int_var);
        "#,).set_name("hiho")?
            .set_environment(env_table)?
            .exec()?;
        Ok(())
    })?;

   Ok(())
}
*/