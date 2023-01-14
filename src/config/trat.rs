use std::collections::HashMap;

pub trait Configuration<'a, K, V> {
    fn map() -> Option<&'a HashMap<K, V>>;
    fn get(&self, key: K) -> Option<&V>;
    fn setup_and_init() -> Result<Self, String>
    where
        Self: Sized;
}

pub trait ConfigurationEditable<'a, K, V> : Configuration<'a, K, V> {
    fn set(key: K, value: V) -> Result<(), String>;
}
