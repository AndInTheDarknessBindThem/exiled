use mod_lookup::ModStore;
use piccolo::{io, Closure, Executor, Lua};
use std::fs::File;

mod item_parser;
mod mod_lookup;
mod lua_struct;
use lua_struct::{LuaStruct, LuaValue};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lua = Lua::empty();

    let file_name = "./data/ModItem.lua";
    let file = io::buffered_read(File::open(file_name)?)?;

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

    // println!("{:#?}", table);
    let raw_item = "Item Class: Sceptres
Rarity: Rare
Blight Song
Shrine Sceptre
--------
Quality: +20% (augmented)
Spirit: 180 (augmented)
--------
Requirements:
Level: 84
Str: 58
Int: 149
--------
Item Level: 84
--------
50% increased Spirit
+28 to maximum Mana
+5 to Level of all Minion Skills
+10 to Intelligence
Minions have 36% increased maximum Life
Allies in your Presence deal 10 to 12 additional Attack Fire Damage
"
    .to_string();

    let item: item_parser::Item = raw_item.try_into()?;

    println!("{:#?}", item);

    for affix in item.affixes {
        println!("[{}]:", affix);
        if let Some((tier, found_mod)) = mods.find(&affix) {
            println!("Tier: {}", tier);
            println!("Max tier: {}", found_mod.iter().max_by_key(|m| m.tier).unwrap().tier);
            let max_ilvl = found_mod.iter().filter(|m| m.ilvl <= item.level).max_by_key(|m| m.tier).unwrap();
            println!("Max at ilvl {}, tier {} (min ilvl {})", item.level, max_ilvl.tier, max_ilvl.ilvl);
            println!("Mod tier: {:#?}", found_mod.iter().find(|m| m.tier == tier));
        }
    }

    Ok(())
}
