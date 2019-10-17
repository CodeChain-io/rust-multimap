use std::collections::{BTreeMap, HashSet};
use std::hash::Hash;
use std::ops::Deref;

#[derive(Debug, Default)]
pub struct MultiMap<K, V>
where
    K: Ord + Eq + Hash,
    V: Eq + Hash, {
    backing: BTreeMap<K, HashSet<V>>,
}

impl<K, V> MultiMap<K, V>
where
    K: Ord + Eq + Hash,
    V: Eq + Hash,
{
    /// Insert an item into the multimap.
    pub fn insert(&mut self, key: K, value: V) -> bool {
        self.backing.entry(key).or_insert_with(Default::default).insert(value)
    }

    /// Remove an item from the multimap.
    /// Returns true if the item was removed successfully.
    pub fn remove(&mut self, key: &K, value: &V) -> bool {
        if let Some(values) = self.backing.get_mut(key) {
            let only_one_left = values.len() == 1;
            if !only_one_left {
                // Operation may be ok: only if value is in values Set.
                return values.remove(value)
            }
            if value
                != values.iter().next().expect("We know there is only one element in collection, tested above; qed")
            {
                // Operation failed: value is not the single item in values Set.
                return false
            }
        } else {
            // Operation failed: value not found in Map.
            return false
        }
        // Operation maybe ok: only if value not found in values Set.
        self.backing.remove(key).is_some()
    }
}

impl<K, V> Deref for MultiMap<K, V>
where
    K: Ord + Eq + Hash,
    V: Eq + Hash,
{
    type Target = BTreeMap<K, HashSet<V>>;

    fn deref(&self) -> &Self::Target {
        &self.backing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut map: MultiMap<u8, u32> = MultiMap::default();
        map.insert(1u8, 3u32);
        map.insert(2u8, 6u32);
        map.insert(1u8, 9u32);
        let set: HashSet<u32> = [3u32, 9u32].iter().cloned().collect();
        assert_eq!(map.get(&1u8).unwrap(), &set);
    }
}
