use chashmap::CHashMap;
use chrono::{DateTime, Utc, Duration};


#[derive(Debug, Clone)]
pub struct TinElement {
    pub data: String,
    pub mimetype: String,
    pub creation: DateTime<Utc>,
    pub update: DateTime<Utc>,
    pub expiration: Option<DateTime<Utc>>,
    pub locked: bool,
}

#[derive(Debug, Clone)]
pub struct TinStore {
    map: CHashMap<String, TinElement>,
}

impl TinStore {
    pub fn new() -> TinStore {
        let store = TinStore {
            map: CHashMap::new(),
        };

        return store;
    }

    pub fn set(&self, key: String, value: String) -> Option<TinElement> {
        match &mut self.map.get_mut(&key) {
            Some(tin_element) => {
                if !tin_element.locked {
                    let mimetype = tree_magic::from_u8(value.as_ref());
                    tin_element.data = value;
                    tin_element.mimetype = mimetype;
                }
                tin_element.update = Utc::now();
                return Some(tin_element.to_owned())
            }
            None => {
                let mimetype = tree_magic::from_u8(value.as_ref());
                let tin_element = TinElement {
                    data: value,
                    mimetype,
                    creation: Utc::now(),
                    update: Utc::now(),
                    expiration: None,
                    locked: false
                };
                return self.map.insert(key, tin_element);
            }
        }
    }

    pub fn set_exp(&self, key: String, value: String, seconds: i64) -> Option<TinElement> {
        match &mut self.map.get_mut(&key) {
            Some(tin_element) => {
                if !tin_element.locked {
                    let mimetype = tree_magic::from_u8(value.as_ref());
                    tin_element.data = value;
                    tin_element.mimetype = mimetype;
                }
                tin_element.update = Utc::now();
                return Some(tin_element.to_owned())
            }
            None => {
                let mimetype = tree_magic::from_u8(value.as_ref());
                let tin_element = TinElement {
                    data: value,
                    mimetype,
                    creation: Utc::now(),
                    update: Utc::now(),
                    expiration: Some(Utc::now() + Duration::seconds(seconds)),
                    locked: false
                };
                return self.map.insert(key, tin_element);
            }
        }
    }

    pub fn get(&self, key: String) -> Option<TinElement> {
        match &mut self.map.get_mut(&key) {
            Some(tin_element) => {
                let cloned = tin_element.clone();
                return Some(cloned);
            }
            None => None,
        }
    }

    pub fn lock(&self, key: String) -> bool {
        match &mut self.map.get_mut(&key) {
            Some(tin_element) => {
                if tin_element.locked {
                    return false;
                }
                tin_element.locked = true;
                return true;
            }
            None => false,
        }
    }

    pub fn unlock(&self, key: String) -> bool {
        match &mut self.map.get_mut(&key) {
            Some(tin_element) => {
                if !tin_element.locked {
                    return false;
                }
                tin_element.locked = false;
                return true;
            }
            None => false,
        }
    }

    pub fn check_expired(&self) {
        self.map.retain(|k, v| {
            self.lock(k.clone());
            match v.expiration {
                Some(date) => {
                    let res = Utc::now() < date;
                    self.unlock(k.clone());
                    return res;
                },
                None => true
            }
        });
    }

    pub fn delete(&self, key: String) -> Option<TinElement> {
       self.map.remove(&key)
    }
}
