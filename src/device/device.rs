use device::driver::Driver;
use config::Config;

use failure::Error;
use rand::{thread_rng, ThreadRng, Rng};

use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Device(usize);

struct DeviceData<'a>(&'a Driver, Config);

pub struct DeviceManager<'a> {
    devices: HashMap<Device, DeviceData<'a>>,
    names: HashMap<Device, String>,
    rng: ThreadRng
}

impl<'a> DeviceManager<'a> {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            names: HashMap::new(),
            rng: thread_rng()
        }
    }

    pub fn add(&mut self, drv: &'a Driver, conf: Config) -> Result<Device, Error> {
        let device = Device(self.devices.len());
        self.devices.insert(device, DeviceData::<'a>(drv, conf));
        let name = self.generate_name();
        self.names.insert(device, name);
        Ok(device)
    }

    fn generate_name(&mut self) -> String {
        let lhs = self.rng.choose(&LHS_WORDS).unwrap();
        let rhs = self.rng.choose(&RHS_WORDS).unwrap();
        let mut name = format!("{}_{}", lhs, rhs);

        let mut count = 0;
        while self.names.values().find(|n| **n == name).is_some() {
            name = format!("{}{}", name, count);
            count += 1;
        }
        name
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
