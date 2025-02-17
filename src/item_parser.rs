use regex::Regex;

#[derive(Debug, Default)]
pub struct Requirements {
    pub level: i64,
    pub str: i64,
    pub dex: i64,
    pub int: i64,
}

#[derive(Debug)]
pub enum Rarity {
    Magic,
    Rare,
    Normal,
    Unique,
    Unknown,
}

#[derive(Debug)]
pub struct Item {
    pub level: i64,
    pub class: Option<String>,
    pub rarity: Rarity,
    pub base: Option<String>,
    pub name: Option<String>,
    pub requirements: Requirements,
    pub affixes: Vec<String>,
    pub special_affixes: Vec<(String, String)>,
    pub corrupted: bool,
    pub socket_count: i64,
    quality: i64,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            level: -1,
            class: None,
            name: None,
            base: None,
            rarity: Rarity::Unknown,
            requirements: Default::default(),
            affixes: Vec::new(),
            special_affixes: Vec::new(),
            corrupted: false,
            socket_count: 0,
            quality: 0,
        }
    }
}

// This will panic on overflow but it shouldn't be a problem for its use case
// https://stackoverflow.com/a/65602018
fn permissive_atoi(s: &str) -> i64 {
    s.chars()
        .map(|c| c.to_digit(10))
        .take_while(|opt| opt.is_some())
        .fold(0, |acc, digit| acc * 10 + digit.unwrap()) as i64
}

impl Rarity {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Rare" => Self::Rare,
            "Magic" => Self::Magic,
            "Normal" => Self::Normal,
            "Unique" => Self::Unique,
            _ => Self::Unknown,
        }
    }
}

// item bases cache goes here

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
                println!("Parsing line: {}", line);
                if let Some(key) = split.next() {
                    if let Some(value) = split.next() {
                        match key {
                            "Rarity" => item.rarity = Rarity::from_str(value),
                            "Sockets" => {
                                item.socket_count =
                                    value.to_string().chars().filter(|c| *c == 'S').count() as i64
                            }
                            "Item Class" => item.class = Some(value.to_string()),
                            "Item Level" => {
                                item.level = permissive_atoi(value);
                            }
                            "Quality" => {
                                item.quality =
                                    permissive_atoi(value.strip_prefix("+").unwrap_or(value))
                            }
                            "Level" => item.requirements.level = permissive_atoi(value),
                            "Str" => item.requirements.str = permissive_atoi(value),
                            "Dex" => item.requirements.dex = permissive_atoi(value),
                            "Int" => item.requirements.int = permissive_atoi(value),
                            _ => {}
                        }
                    } else if item.level == -1 && !line.is_empty() && !line.contains(":") {
                        if let Some(_) = item.name {
                            item.base = Some(line.to_string());
                        } else {
                            item.name = Some(line.to_string());
                        }
                    } else if item.level > -1 && !line.is_empty() {
                        // Lines after item level are affixes or Corrupted
                        if line.eq("Corrupted") {
                            item.corrupted = true;
                        } else {
                            let affix_re =
                                Regex::new(r"^(?<affix>.+?)( \((?<special>[^\)]+)\))?$").unwrap();

                            if let Some(captures) = affix_re.captures(line) {
                                if let (Some(affix), Some(special)) =
                                    (captures.name("affix"), captures.name("special"))
                                {
                                    item.special_affixes.push((
                                        special.as_str().to_owned(),
                                        affix.as_str().to_owned(),
                                    ));
                                } else {
                                    item.affixes.push(line.to_string());
                                }
                            }
                        }
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
