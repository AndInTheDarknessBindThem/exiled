#[derive(Debug, Default)]
pub struct Requirements {
    pub level: i64,
    pub str: i64,
    pub dex: i64,
    pub int: i64,
}

#[derive(Debug)]
pub struct Item {
    pub level: i64,
    pub class: Option<String>,
    pub name: Option<String>,
    pub requirements: Requirements,
    pub affixes: Vec<String>,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            level: -1,
            class: None,
            name: None,
            requirements: Default::default(),
            affixes: Vec::new(),
        }
    }
}

impl Item {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_from_string(s: &String) -> Result<Self, String> {
        let mut item = Item::new();

        s.as_str()
            .split("\n")
            .filter(|line| !line.eq(&"--------"))
            .for_each(|line| {
                let mut split = line.split(": ");
                if let Some(key) = split.next() {
                    if let Some(value) = split.next() {
                        match key {
                            "Item Class" => item.class = Some(value.to_string()),
                            "Item Level" => {
                                item.level = value.parse::<i64>().unwrap_or(0);
                            }
                            "Level" => item.requirements.level = value.parse::<i64>().unwrap_or(0),
                            "Str" => item.requirements.str = value.parse::<i64>().unwrap_or(0),
                            "Dex" => item.requirements.dex = value.parse::<i64>().unwrap_or(0),
                            "Int" => item.requirements.int = value.parse::<i64>().unwrap_or(0),
                            _ => {}
                        }
                    } else if item.level > -1 && !line.is_empty() {
                        item.affixes.push(line.to_string());
                    }
                }
            });

        Ok(item)
    }
}

impl TryFrom<String> for Item {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new_from_string(&value)
    }
}
