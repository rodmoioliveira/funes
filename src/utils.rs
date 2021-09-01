use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub fn sep_hash(qs: &str) -> &str {
    match qs == "" {
        true => "",
        false => "-",
    }
}

pub fn sep_qs(qs: &str) -> &str {
    match qs == "" {
        true => "",
        false => "?",
    }
}

pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(PartialEq)]
pub struct HashValue<'a>(pub &'a serde_json::Value);

impl Eq for HashValue<'_> {}

impl Hash for HashValue<'_> {
    /// Implements the [`Hash`][Hash] trait to
    /// [`serde_json::Value`][serde_json::Value]
    fn hash<H: Hasher>(&self, state: &mut H) {
        use serde_json::Value::*;
        match self.0 {
            Null => state.write_u32(3_221_225_473), // chosen randomly
            Bool(ref b) => b.hash(state),
            Number(ref n) => {
                if let Some(x) = n.as_u64() {
                    x.hash(state);
                } else if let Some(x) = n.as_i64() {
                    x.hash(state);
                } else if let Some(x) = n.as_f64() {
                    ordered_float::NotNan::new(x).unwrap().hash(state);
                }
            }
            String(ref s) => s.hash(state),
            Array(ref v) => {
                for x in v {
                    HashValue(x).hash(state);
                }
            }
            Object(ref map) => {
                let mut hash = 0;
                for (k, v) in map {
                    let mut item_hasher = DefaultHasher::new();
                    k.hash(&mut item_hasher);
                    HashValue(v).hash(&mut item_hasher);
                    hash ^= item_hasher.finish();
                }
                state.write_u64(hash);
            }
        }
    }
}
