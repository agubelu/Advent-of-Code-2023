use rustc_hash::FxHashMap;
use num_traits::int::PrimInt;
use std::hash::Hash;

pub struct IDAssigner<K: Hash + Eq, V: PrimInt> {
    data: FxHashMap<K, V>,
}

impl<K: Hash + Eq, V: PrimInt> IDAssigner<K, V> {
    pub fn new() -> Self {
        Self { data: FxHashMap::default() }
    }

    pub fn get_id(&mut self, elem: K) -> V {
        if let Some(id) = self.data.get(&elem) {
            *id
        } else {
            let next_id = V::from(self.data.len()).unwrap();
            self.data.insert(elem, next_id);
            next_id
        }
    }
}
