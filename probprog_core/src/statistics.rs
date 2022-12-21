use std::collections::BTreeMap;

pub fn occurences<B: Ord>(vec: Vec<B>) -> BTreeMap<B, usize> {
    let mut occurences: BTreeMap<B, usize> = BTreeMap::new();
    for b in vec {
        occurences.entry(b).and_modify(|c| *c += 1).or_insert(0);
    }
    occurences
}

pub fn normalize<B: Ord>(occurences: BTreeMap<B, usize>) -> BTreeMap<B, f64> {
    let total = occurences.values().sum::<usize>() as f64;
    let mut normalized_occurences: BTreeMap<B, f64> = BTreeMap::new();
    for (k, v) in occurences {
        let n = (v as f64) / total;
        normalized_occurences.insert(k, n);
    }

    normalized_occurences
}
