use global_hotkey::{hotkey::{Code, HotKey, Modifiers}, GlobalHotKeyEvent, GlobalHotKeyManager};

slint::include_modules!();

struct Mod {
}

enum Rarity {
    Normal,
    Magic,
    Rare,
    Unique,
    Currency,
    Gem,
    Quest,
    Other(String),
}

struct Item {
    class: String,
    rarity: String,
    item_name: Option<String>,
    base_name: String,
    basic_stats: Vec<String>,
    requirements: Vec<String>,
    item_level: u64,
    implicit: Vec<Mod>,
    mods: Vec<Mod>,
}

fn main() {
    // initialize the hotkeys manager
    let manager = GlobalHotKeyManager::new().unwrap();

    // construct the hotkey
    let hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::KeyD);

    // register it
    manager.register(hotkey).unwrap();

    std::thread::spawn(|| {
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            println!("{:?}", event);
        }
    });

    ExiledMainWindow::new().unwrap().run().unwrap();
}

fn parse_item(item: &str) -> Item {
    enum State {
        Start,
        ItemName,
        BaseStats,
        Requirements,
        Mods,
    };
    let mut state = State::Start;
    let mut item_class = None;
    let mut rarity = None;
    let mut item_name = None;
    let mut base_name = None;
    let mut item_level = None;
    let mut level_requirement = None;
    let mut int_requirement = None;
    let mut str_requirement = None;
    let mut dex_requirement = None;
    for line in item.lines() {
        if line.starts_with("----") {
            match state {
                State::Start | State::ItemName => state = State::BaseStats,
                State::BaseStats => state = State::Requirements,
                State::Requirements => state = State::Mods,
                _ => (),
            }
            continue;
        }
        if line.starts_with("Requirements:") {
            state = State::Requirements;
        }
        if !line.contains(":") {
            match state {
                State::Start => {
                    base_name = Some(line.to_string());
                    state = State::ItemName;
                },
                State::ItemName => {
                    item_name = base_name.take();
                    base_name = Some(line.to_string());
                },
                _ => (),
            }
        }
        let (before, after) = line.split_once(":").unwrap();
        match before {
            "Item Class" => {
                item_class = Some(after.to_string());
                continue;
            }
            "Rarity" => {
                rarity = Some(after.to_string());
                continue;
            }
            "Level" => {
                level_requirement = Some(after.parse());
                continue;
            }
            "Item Level" => {
                item_level = Some(after.parse());
                continue;
                // TODO: Mods are coming after this.
            },
            _ => (),
        }
    }

    Item {
        class: item_class.unwrap(),
        item_level: item_level.unwrap(),
    rarity: String,
    item_name: Option<String>,
    base_name: String,
    basic_stats: Vec<String>,
    requirements: Vec<String>,
    implicit: Vec<Mod>,
    mods: Vec<Mod>,
    }
}