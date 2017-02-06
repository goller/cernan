use metric::TagMap;
use time;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct LogLine {
    pub time: i64,
    pub path: String,
    pub value: String,
    pub tags: TagMap,
}

impl LogLine {
    pub fn new<S>(path: S, value: S) -> LogLine
        where S: Into<String>
    {
        LogLine {
            path: path.into(),
            value: value.into(),
            time: time::now(),
            tags: Default::default(),
        }
    }

    pub fn time(mut self, time: i64) -> LogLine {
        self.time = time;
        self
    }

    pub fn overlay_tag<S>(mut self, key: S, val: S) -> LogLine
        where S: Into<String>
    {
        self.tags.insert(key.into(), val.into());
        self
    }

    pub fn overlay_tags_from_map(mut self, map: &TagMap) -> LogLine {
        for &(ref k, ref v) in map.iter() {
            self.tags.insert(k.clone(), v.clone());
        }
        self
    }
}
