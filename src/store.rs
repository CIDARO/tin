use chashmap::CHashMap;
use chrono::{DateTime, Utc, Duration};
use short_crypt::ShortCrypt;
use std::str;


#[derive(Debug, Clone, Serialize)]
pub struct TinElement {
    pub data: String,
    pub creation: DateTime<Utc>,
    pub update: DateTime<Utc>,
    pub expiration: Option<DateTime<Utc>>,
    pub locked: bool,
}

#[derive(Debug, Clone)]
pub struct TinStore {
    map: CHashMap<String, TinElement>,
    key: String
}

impl TinStore {
    pub fn new(key: String) -> Self {
        Self {
            map: CHashMap::new(),
            key,
        }
    }

    pub fn set(&self, key: String, value: String) -> Option<TinElement> {
        match &mut self.map.get_mut(&key) {
            Some(tin_element) => {
                if !tin_element.locked {
                    if self.key != "" {
                        let sc = ShortCrypt::new(self.key.clone());
                        tin_element.data = sc.encrypt_to_url_component(value.clone().as_str());
                    } else {
                        tin_element.data = value;
                    }
                    tin_element.update = Utc::now();
                }
                return Some(tin_element.to_owned())
            }
            None => {
                let tin_element = TinElement {
                    data: value,
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
                    if self.key != "" {
                        let sc = ShortCrypt::new(self.key.clone());
                        tin_element.data = sc.encrypt_to_url_component(value.clone().as_str());
                    } else {
                        tin_element.data = value;
                    }
                    tin_element.update = Utc::now();
                }
                return Some(tin_element.to_owned())
            }
            None => {
                let tin_element = TinElement {
                    data: value,
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
                let mut cloned = tin_element.clone();
                if self.key != "" {
                    let sc = ShortCrypt::new(self.key.clone());
                    cloned.data = str::from_utf8(&sc.decrypt_url_component(cloned.clone().data).unwrap()).unwrap().to_string();
                }
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

    pub async fn check_expired(&self) {
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
