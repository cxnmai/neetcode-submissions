use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, TimeMapValue>,
}

struct TimeMapValue {
    values: Vec<String>,
    timestamps: Vec<i32>,
}


impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        if let Some(entry) = self.map.get_mut(&key) {
            entry.values.push(value);
            entry.timestamps.push(timestamp);
        } else {
            self.map.insert(key, TimeMapValue {
                values: vec![value],
                timestamps: vec![timestamp],
            },
            );
        }
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let val = match self.map.get(&key) {
            Some(thing) => thing,
            None => return "".to_string(),
        };

        let mut s = 0;
        let mut e = val.timestamps.len();

        let mut i = (s + e) / 2;

        while s < e {
            if val.timestamps[s] > timestamp {
                break;
            }
            i = (s + e) / 2;

            if val.timestamps[i] == timestamp {
                break;
            } else if val.timestamps[i] < timestamp {
                s = i + 1;
                
            } else {
                e = i;
            }
        }

        if val.timestamps[i] > timestamp {
            return "".to_string();
        }

        val.values[i].clone()

    }
}
