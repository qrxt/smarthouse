use std::collections::HashMap;

pub fn has_duplicates(items: &[String]) -> bool {
    let mut hashmap: HashMap<String, i32> = HashMap::new();

    items.iter().for_each(|item| {
        if !hashmap.contains_key(item) {
            hashmap.insert(item.to_string(), 1);
        } else {
            hashmap.insert(item.to_string(), hashmap[item] + 1);
        }
    });

    hashmap.values().into_iter().any(|freq| freq > &1)
}
