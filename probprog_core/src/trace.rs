use crate::{bernoulli::Bernoulli, distribution::Distribution};

#[derive(Debug)]
pub struct TraceEntryValues<ParamsType, SupportType> {
    pub params: ParamsType,
    pub value: SupportType,
    pub log_likelihood: f64,
}

#[derive(Debug)]
pub enum TraceEntry {
    Bernoulli(
        TraceEntryValues<
            <Bernoulli as Distribution>::ParamsType,
            <Bernoulli as Distribution>::SupportType,
        >,
    ),
}

// pub struct TraceEntry2<ParamsType, SupportType> {
//     pub dist:
//         dyn Distribution<ParamsType = ParamsType, SupportType = SupportType>,
//     pub value: SupportType,
//     pub log_likelihood: f64,
// }

// pub struct Trace2 {
//     tree: BTreeMap<String, TraceEntry2<_, _>>,
// }

// #[derive(Debug)]
// pub struct Trace {
//     tree: BTreeMap<String, TraceEntry>,
// }

// impl Trace {
//     pub fn new() -> Self {
//         Self {
//             tree: BTreeMap::new(),
//         }
//     }

//     pub fn insert(
//         &mut self,
//         key: String,
//         value: TraceEntry,
//     ) -> Option<TraceEntry> {
//         self.tree.insert(key, value)
//     }

//     pub fn get(&self, key: &String) -> Option<&TraceEntry> {
//         self.tree.get(key)
//     }
// }
