use device::driver::{Driver, DriverData};
use device::driver_manager::DriverManager;
use config::Config;

use failure::Error;
use rand::{thread_rng, ThreadRng, Rng};

use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Device(usize);

struct DeviceData(Driver, Config);

pub struct DeviceManager {
    driver_manager: DriverManager,
    devices: HashMap<Device, DeviceData>,
    names: HashMap<Device, String>,
    rng: ThreadRng
}

impl DeviceManager {
    pub fn new() -> Self {
        let mut inst = Self {
            driver_manager: DriverManager::new(),
            devices: HashMap::new(),
            names: HashMap::new(),
            rng: thread_rng()
        };
        inst.driver_manager.load_all();
        inst
    }

    pub fn driver_manager(self) -> DriverManager {
        self.driver_manager
    }

    pub fn detect(&mut self, conf: Config) -> Result<Vec<Device>, Error> {
        let &mut Self { ref mut devices, ref mut names, ref mut driver_manager, ref mut rng, .. } = self;
        driver_manager.driver_data()
            .map(|(drv, data)| Ok((*drv, data.detect(&conf)?)))
            .collect::<Result<HashMap<Driver, Vec<Config>>, Error>>()
            .map(|v| {
                v.into_iter()
                .flat_map(|(drv, confs)| confs.into_iter().map(|c| {
                    let device = Device(devices.len());
                    devices.insert(device, DeviceData(drv, c));
                    let lhs = rng.choose(&LHS_WORDS).unwrap();
                    let rhs = rng.choose(&RHS_WORDS).unwrap();
                    let mut name = format!("{}_{}", lhs, rhs);

                    let mut count = 0;
                    while names.values().find(|n| **n == name).is_some() {
                        name = format!("{}{}", name, count);
                        count += 1;
                    }
                    names.insert(device, name);
                    device
                }).collect::<Vec<_>>())
                .collect()
            })
    }

    pub fn get_device_name(&self, dev: &Device) -> Option<&String> {
        self.names.get(dev)
    }

    pub fn get_device_config(&self, dev: &Device) -> Option<&Config> {
        self.devices.get(dev).map(|data| &data.1)
    }

    pub fn get_name_device(&self, name: &str) -> Option<&Device> {
        self.names.iter().find(|(k, v)| *v == name).map(|v| v.0)
    }
}

const LHS_WORDS: [&str; 100] = [
  "other",
  "new",
  "good",
  "high",
  "old",
  "great",
  "big",
  "merican",
  "small",
  "large",
  "national",
  "young",
  "different",
  "black",
  "long",
  "little",
  "important",
  "political",
  "bad",
  "white",
  "real",
  "best",
  "right",
  "social",
  "only",
  "public",
  "sure",
  "low",
  "early",
  "able",
  "human",
  "local",
  "late",
  "hard",
  "major",
  "better",
  "economic",
  "strong",
  "possible",
  "whole",
  "free",
  "military",
  "true",
  "federal",
  "international",
  "full",
  "special",
  "easy",
  "clear",
  "recent",
  "certain",
  "personal",
  "open",
  "red",
  "difficult",
  "available",
  "likely",
  "short",
  "single",
  "medical",
  "current",
  "wrong",
  "private",
  "past",
  "foreign",
  "fine",
  "common",
  "poor",
  "natural",
  "significant",
  "similar",
  "hot",
  "dead",
  "central",
  "happy",
  "serious",
  "ready",
  "simple",
  "left",
  "physical",
  "general",
  "environmental",
  "financial",
  "blue",
  "democratic",
  "dark",
  "various",
  "entire",
  "close",
  "legal",
  "religious",
  "cold",
  "final",
  "main",
  "green",
  "nice",
  "huge",
  "popular",
  "traditional",
  "cultural",
];

const RHS_WORDS: [&str; 100] = [
  "time",
  "year",
  "people",
  "way",
  "day",
  "man",
  "thing",
  "woman",
  "life",
  "child",
  "world",
  "school",
  "state",
  "family",
  "student",
  "group",
  "country",
  "problem",
  "hand",
  "part",
  "place",
  "case",
  "week",
  "company",
  "system",
  "program",
  "question",
  "work",
  "government",
  "number",
  "night",
  "point",
  "home",
  "water",
  "room",
  "mother",
  "area",
  "money",
  "story",
  "fact",
  "month",
  "lot",
  "right",
  "study",
  "book",
  "eye",
  "job",
  "word",
  "business",
  "issue",
  "side",
  "kind",
  "head",
  "house",
  "service",
  "friend",
  "father",
  "power",
  "hour",
  "game",
  "line",
  "end",
  "member",
  "law",
  "car",
  "city",
  "community",
  "name",
  "president",
  "team",
  "minute",
  "idea",
  "kid",
  "body",
  "information",
  "back",
  "parent",
  "face",
  "others",
  "level",
  "office",
  "door",
  "health",
  "person",
  "art",
  "war",
  "history",
  "party",
  "result",
  "change",
  "morning",
  "reason",
  "research",
  "girl",
  "guy",
  "moment",
  "air",
  "teacher",
  "force",
  "education",
];
