use piccolo::{io, Closure, Executor, Lua, Table, Value, FromMultiValue};
use std::{collections::HashMap, fs::File};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lua = Lua::empty();

    let file_name = "data.lua";
    let file = io::buffered_read(File::open(file_name)?)?;

    let executor = lua.try_enter(|ctx| {
        let closure = Closure::load(ctx, Some(file_name), file)?;
        Ok(ctx.stash(Executor::start(ctx, closure.into(), ())))
    })?;
/*
    lua.finish(&executor);

    lua.try_enter(|ctx| {
        ctx.fetch(&executor).take_result::<Table>(ctx)?
    });
*/
    let (table,) = lua.execute::<(MyStruct,)>(&executor)?;

    Ok(())
}

enum MyValue {
    String(String),
    Number(u64),
    List(Vec<MyValue>),
    Table(Box<MyValue>),
}

struct MyStruct {
    data: HashMap<String, MyValue>
}

fn convert_table(table: Table<'_>) -> MyStruct {
    let mystruct = MyStruct {
        data: Default::default()
    };
    for (k, v) in table.iter() {
        let k = match k {
            Value::Integer(k) => k.to_string(),
            Value::String(k) => k.to_string(),
            _ => todo!(),
        };
        let v = match v {
            Value::Nil => todo!(),
            Value::Boolean(_) => todo!(),
            Value::Integer(_) => todo!(),
            Value::Number(_) => todo!(),
            Value::String(_) => todo!(),
            Value::Table(table) => Box::new(convert_table(table)),
            _ => todo!(),
        };
    }
    mystruct
}

impl<'gc> FromMultiValue<'gc> for MyStruct {
    fn from_multi_value(
        ctx: piccolo::Context<'gc>,
        values: impl Iterator<Item = Value<'gc>>,
    ) -> Result<Self, piccolo::TypeError> {
        let table = Table::from_multi_value(ctx, values)?;
        Ok(convert_table(table))
    }
}