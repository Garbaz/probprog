use std::{collections::BTreeMap, ops::Range};

pub fn occurences<A, D>(data: D) -> BTreeMap<A, usize>
where
    A: Ord,
    D: IntoIterator<Item = A>,
{
    let mut occurences: BTreeMap<A, usize> = BTreeMap::new();
    for x in data {
        occurences.entry(x).and_modify(|c| *c += 1).or_insert(0);
    }
    occurences
}

pub fn densities<F, D>(range: Range<F>, buckets: usize, data: D) -> Vec<usize>
where
    F: Into<f64> + Copy,
    D: IntoIterator<Item = F>,
{
    let mut occurences = vec![0; buckets];
    for x in data {
        let start: f64 = range.start.into();
        let end: f64 = range.end.into();
        let x: f64 = x.into();
        let p = ((x - start) / (end - start)) * (buckets as f64);
        let i = (p as usize).clamp(0, buckets);
        occurences[i] += 1;
    }
    occurences
}

pub fn normalize_map<A: Ord>(
    occurences: BTreeMap<A, usize>,
) -> BTreeMap<A, f64> {
    let total = occurences.values().sum::<usize>() as f64;
    let mut normalized_occurences: BTreeMap<A, f64> = BTreeMap::new();
    for (k, c) in occurences {
        let n = (c as f64) / total;
        normalized_occurences.insert(k, n);
    }

    normalized_occurences
}

pub fn normalize<O>(occurences: O) -> Vec<f64>
where
    O: IntoIterator<Item = usize>,
{
    let mut occurences: Vec<f64> =
        occurences.into_iter().map(|c| (c as f64)).collect();
    let total: f64 = occurences.iter().sum();
    for c in &mut occurences {
        *c /= total;
    }
    occurences
}
