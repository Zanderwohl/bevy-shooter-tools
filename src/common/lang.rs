use std::collections::HashMap;
use std::fs;
use std::sync::{RwLock};

use lazy_static::lazy_static;
use toml::Value;
use regex::Regex;

// Someday, I'd really love to actually parse the language file in and split it by each {}
// then sort the {names} so that they're just in the order they go in the string.
// Finally, when we want a string we "ask" the caller for each key in order
// (by this I mean look up the key in the dict first to last)
// This way I hope we spend less time making and hashing strings
// but I'm not actually sure if this would speed up the hashmap lookup.
// Perhaps some kind of tokenization optimization is in our future?
//
// Right now we just expand the cache forever. Leaky!
lazy_static! {
    static ref LANG: RwLock<Value> = RwLock::new(load_lang("en-US"));

    static ref NAME_PATTERN: String = r"[A-Za-z_]+".into();
    static ref TEMPLATE_REGEX: Regex = Regex::new(r"\$\{\s*[A-Za-z_]+\s*\}").unwrap();
    static ref NAME_REGEX: Regex = Regex::new(&NAME_PATTERN).unwrap();

    static ref CACHE: HashMap<String, String> = HashMap::new();
}

pub fn change_lang(lang: &str) -> Result<(), String> {
    let mut lang_lock = LANG.write().map_err(|_| "Could not change language; could not acquire lock.")?;
    *lang_lock = load_lang(lang);
    Ok(())
}

fn load_lang(lang: &str) -> Value {
    let pack = "default";
    let lang_path = format!("assets/{}/lang/{}.toml", pack, lang);
    let toml_str = fs::read_to_string(&lang_path)
        .expect(format!("Lang file \"{}\" not found!", lang_path).as_str());

    let parsed = toml_str
        .parse()
        .expect(format!("Could not parse lang file \"{}\"!", lang_path).as_str());

    parsed
}

pub fn get_maybe(keys: &[&str]) -> Option<String> {
    let mut current: &Value = &*(LANG.read().unwrap());
    for key in keys {
        let next_current = current.get(*key);
        match next_current {
            Some(next_current) => current = next_current,
            None => return None
        }
    }
    match current.as_str() {
        None => None,
        Some(s) => Some(s.to_string())
    }
}

pub fn get_infallible(keys: &[&str]) -> String {
    get_maybe(keys).unwrap_or(format!("<{}>", full_path(keys)))
}

pub fn get_parsed(keys: &str) -> String {
    let keys = keys.split('.').collect::<Vec<&str>>();
    get_infallible(&keys)
}

fn full_path(keys: &[&str]) -> String {
    let mut debug_string = "".into();
    for (idx, key) in keys.iter().enumerate() {
        debug_string += *key;
        if idx < keys.len() - 1 {
            debug_string += "."
        }
    }
    debug_string
}

fn pair_path(pairs: &[(&str, &str)]) -> String {
    pairs.iter().map(|(k, v)| format!("{}:{}", k, v)).collect::<Vec<_>>().join("::")
}

fn template_pair_path(keys: &[&str], pairs: &[(&str, &str)]) -> String {
    format!("{}|{}", full_path(keys), pair_path(pairs))
}

pub fn get_template_maybe(keys: &[&str], pairs: &[(&str, &str)]) -> Result<String, String> {
    let cache_key = template_pair_path(keys, pairs);
    match CACHE.get(&cache_key) {
        Some(cached_value) => Ok(cached_value.clone()),
        None => fill_template(keys, pairs, true),
    }
}

pub fn get_template(keys: &[&str], pairs: &[(&str, &str)]) -> String {
    let cache_key = template_pair_path(keys, pairs);
    match CACHE.get(&cache_key) {
        Some(cached_value) => cached_value.clone(),
        None => {
            let pairs_string = format!("{:?}", pairs);
            fill_template(keys, pairs, false).unwrap_or(format!("<{}; {}>", full_path(keys), pairs_string))
        }
    }
}

pub fn get_template_parsed(key: &str, pairs: &[(&str, &str)]) -> String {
    let keys = key.split('.').collect::<Vec<&str>>();
    get_template(&keys, pairs)
}

#[macro_export]
macro_rules! get {
    ($key:expr) => {
        $crate::common::lang::get_parsed(&$key)
    };
    ($key:expr, $($var:expr, $val:expr),*) => {
        $crate::common::lang::get_template_parsed(&$key, &[
            $(($var, &format!("{}", $val))),*
        ])
    };
}

fn fill_template(keys: &[&str], pairs: &[(&str, &str)], fail_quickly: bool) -> Result<String, String> {
    let template = get_maybe(keys).ok_or(format!("Failed to find value for key {:?}", keys))?;

    let pairs: HashMap<String, String> = pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();

    let mut product = template.clone();

    for (key, replacement) in pairs {
        let re = Regex::new(&format!(r"\{{\s*{}\s*\}}", key));
        match re {
            Ok(re) => {
                product = re.replace_all(&product, replacement.as_str()).parse().unwrap();
            }
            Err(_) => {
                if fail_quickly {
                    return Err(format!("Replacement names must be valid regex: \"{}\"", key));
                }
            }
        }
    }

    Ok(product)
}
