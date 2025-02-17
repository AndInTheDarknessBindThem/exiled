use std::collections::HashMap;

use piccolo::{FromMultiValue, Lua, Table, Value};

#[derive(Debug)]
pub enum LuaValue {
    String(String),
    Number(f64),
    Integer(i64),
    List(Vec<LuaValue>),
    Table(Box<LuaStruct>),
    Boolean(bool),
}

impl LuaValue {
    pub fn as_int(&self) -> Option<i64> {
        if let LuaValue::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct LuaStruct {
    pub data: HashMap<String, LuaValue>,
    pub list: Vec<LuaValue>,
}

impl From<Table<'_>> for LuaStruct {
    fn from(value: Table<'_>) -> Self {
        let mut lua_struct = LuaStruct {
            data: Default::default(),
            list: Default::default(),
        };
        for (k, v) in value.iter() {
            let v = match v {
                Value::Integer(_) => LuaValue::Integer(v.to_integer().unwrap_or(0)),
                Value::Number(_) => LuaValue::Number(v.to_number().unwrap_or(0 as f64)),
                Value::String(_) => LuaValue::String(v.to_string()),
                Value::Table(table) => LuaValue::Table(Box::new(table.into())),
                Value::Boolean(_) => LuaValue::Boolean(v.to_bool()),
                _ => todo!(),
            };

            match k {
                Value::Integer(_) => {
                    lua_struct.list.push(v);
                    None
                },
                Value::String(k) => lua_struct.data.insert(k.to_string(), v),
                _ => todo!(),
            };
        }
        lua_struct
    }
}

impl<'gc> FromMultiValue<'gc> for LuaStruct {
    fn from_multi_value(
        ctx: piccolo::Context<'gc>,
        values: impl Iterator<Item = Value<'gc>>,
    ) -> Result<Self, piccolo::TypeError> {
        let table = Table::from_multi_value(ctx, values)?;
        Ok(table.into())
    }
}

impl LuaStruct {
    pub fn get(&self, key: &str) -> Option<&LuaValue> {
        self.data.get(key)
    }

    pub fn get_index(&self, index: usize) -> Option<&LuaValue> {
        self.list.get(index)
    }
}