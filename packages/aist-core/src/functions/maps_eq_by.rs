use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

pub fn maps_eq_by<K: Eq + Hash, L, R, S: BuildHasher, T: BuildHasher>(left: &HashMap<K, L, S>, right: &HashMap<K, R, T>, mut equal: impl FnMut(&L, &R) -> bool) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .all(|(key, left)| right.get(key).is_some_and(|right| equal(left, right)))
}
