use regex::Regex;
use std::{collections::HashMap, ops::RangeInclusive};

use crate::lua_struct::{LuaStruct, LuaValue};

#[derive(Debug)]
pub enum AffixType {
    Prefix,
    Suffix,
    Unknown,
}

#[derive(Debug)]
pub struct Mod {
    pub ilvl: i64,               // item level
    pub values: Vec<RangeInclusive<f64>>, // (min, max), empty if no variance
    pub tier: i64,               // mod tier
    pub mod_hash: i64, // hash of the mod, to differentiate between mods with the same affix
    pub affix_type: AffixType, // prefix or suffix
    pub pob_name: String, // name of the mod in Path of Building
}

pub struct ModStore {
    mods: HashMap<String, Vec<Mod>>,
}

impl ModStore {
    pub fn new() -> ModStore {
        ModStore {
            mods: HashMap::new(),
        }
    }

    pub fn insert(&mut self, pob_name: &String, data: &LuaStruct) {
        for value in data.list.iter() {
            if let LuaValue::String(affix) = value {
                let re = Regex::new(r"-?\((?<min>-?\d+(\.\d+)?)-(?<max>-?\d+(\.\d+)?)\)|(\d+)").unwrap();

                let stripped_affix = re.replace_all(&affix, "#").into_owned();
                let pob_name_re = Regex::new(r"(?<name>.+)(?<tier>\d+)$").unwrap();

                let captures = pob_name_re.captures(pob_name);

                let mut mod_entry = Mod {
                    ilvl: data
                        .get("level")
                        .and_then(|v| v.as_int())
                        .unwrap_or_default(),
                    tier: captures
                        .as_ref()
                        .and_then(|caps| caps.name("tier"))
                        .and_then(|m| m.as_str().parse::<i64>().ok())
                        .unwrap_or_default(),
                    pob_name: captures
                        .as_ref()
                        .and_then(|caps| caps.name("name"))
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default(),
                    values: Default::default(),
                    affix_type: match data.get("type") {
                        Some(LuaValue::String(s)) if s == "Suffix" => AffixType::Suffix,
                        Some(LuaValue::String(s)) if s == "Prefix" => AffixType::Prefix,
                        _ => AffixType::Unknown,
                    },
                    mod_hash: data
                        .get("tradeHash")
                        .and_then(|v| v.as_int())
                        .unwrap_or_default(),
                };

                re.captures_iter(&affix)
                    .map(|cap| {
                        if let (Some(min), Some(max)) = (cap.name("min"), cap.name("max")) {
                            let min = min.as_str().parse::<f64>().unwrap_or(0.0);
                            let max = max.as_str().parse::<f64>().unwrap_or(0.0);
                            min..=max
                        } else if let Some(val) = cap.get(5) {
                            let minmax = val.as_str().parse::<f64>().unwrap_or(0.0);
                            minmax..=minmax
                        } else {
                            0.0..=0.0
                        }
                    })
                    .for_each(|v| mod_entry.values.push(v));

                self.mods
                    .entry(stripped_affix)
                    .or_insert(Vec::new())
                    .push(mod_entry);
            }
        }
    }

    pub fn find(&self, mod_line: &str) -> Option<(i64, &Vec<Mod>)> {
        let re = Regex::new(r"(-?\d+(\.\d+)?)").unwrap();
        let stripped_mod_line = re.replace_all(mod_line, "#").into_owned();
        let mut values = Vec::new();

        for cap in re.captures_iter(mod_line) {
            if let Some(val) = cap.get(1) {
                values.push(val.as_str().parse::<f64>().unwrap_or(0.0));
            }
        }

        if let Some(found_mod) = self.mods.get(&stripped_mod_line) {
            if let Some(matching_mod) = found_mod.iter().find(|m| {
                if m.values.len() != values.len() {
                    return false;
                }
                values
                    .iter()
                    .zip(m.values.iter())
                    .all(|(value, range)| range.contains(value))
            }) {
                Some((matching_mod.tier, found_mod))
            } else {
                Some((0, found_mod))
            }
        } else {
            None
        }
    }
}
