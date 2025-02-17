use mod_lookup::ModStore;
use piccolo::{io, Closure, Executor, Lua, Value};
use std::fs::{self, File};

mod item_parser;
mod lua_struct;
mod mod_lookup;
use lua_struct::{LuaStruct, LuaValue};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let item_file = std::env::args()
        .nth(1)
        .map(|f| format!("samples/{}", f))
        .unwrap_or_else(|| "samples/default.item".to_string());

    println!("Reading {}...", item_file);

    let raw_item = std::fs::read_to_string(item_file)?;

    let item: item_parser::Item = raw_item.try_into()?;

    println!("{:#?}", item);


    let file_name = "./data/bases/body.lua";
    let file_content = fs::read_to_string(file_name)?;
    // let mut content = String::from_utf8_lossy(&file_content.as_bytes()).replace("...", "{}");
    // content += "\nreturn itemBases";

    let mut lua = Lua::empty();

    let lua_bases = lua.try_enter(|ctx| {
        Ok(ctx.stash(piccolo::Table::new(&ctx)))
    })?;

    let executor = lua.try_enter(|ctx| {
        let closure: Closure<'_> = Closure::load(ctx, Some(file_name), file_content.as_bytes())?;
        Ok(ctx.stash(Executor::start(ctx, closure.into(), ctx.fetch(&lua_bases))))
    })?;

    lua.execute(&executor)?;

    let lua_bases = lua.try_enter(|ctx| {
        let table = ctx.fetch(&lua_bases);
        Ok(LuaStruct::from(table))
    })?;

    println!("{:#?}", lua_bases);

    /*
    let file_name = "./data/ModItem.lua";
    let file = io::buffered_read(File::open(file_name)?)?;


    let mut lua = Lua::empty();
    let executor = lua.try_enter(|ctx| {
        let closure = Closure::load(ctx, Some(file_name), file)?;
        Ok(ctx.stash(Executor::start(ctx, closure.into(), ())))
    })?;

    let (table,) = lua.execute::<(LuaStruct,)>(&executor)?;

    let mut mods = ModStore::new();

    table.data.iter().for_each(|(k, v)| {
        if let LuaValue::Table(data) = v {
            mods.insert(k, data);
        }
    });

    for affix in item.affixes {
        println!("[{}]:", affix);
        if let Some((tier, found_mod)) = mods.find(&affix) {
            println!("Tier: {}", tier);
            println!("Max tier: {}", found_mod.iter().max_by_key(|m| m.tier).unwrap().tier);
            let max_ilvl = found_mod.iter().filter(|m| m.ilvl <= item.level).max_by_key(|m| m.tier).unwrap();
            println!("Max at ilvl {}, tier {} (min ilvl {})", item.level, max_ilvl.tier, max_ilvl.ilvl);
            //println!("Mod tier: {:#?}", found_mod.iter().find(|m| m.tier == tier));
        }
    }*/

    Ok(())
}
