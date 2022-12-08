use probprog::{
    bernoulli::{Bernoulli, BernoulliParams},
    distribution::Distribution,
    trace::{Database, DatabaseEntry, TraceEntry},
};

/// A mock-up of how a probabilstic function would end up looking like
/// after being transformed by the macro.
/// Note: We should extract as much as possible from the function itself
/// into pre-written functions, such that the macro shenanigans are kept
/// at a minimum.
fn probfunc(tracedb: &mut Database) -> u8 {
    let x = {
        let params_c = BernoulliParams { p: 0.5 };
        let name = "0".to_string();
        let value_t = match tracedb.get(&name) {
            Some(DatabaseEntry {
                trace_entry: TraceEntry::Bernoulli(params, value),
                likelihood,
            }) if *params == params_c => *value,
            _ => {
                let d = Bernoulli::new(params_c).unwrap();
                let value = d.sample();
                let trace_entry = TraceEntry::Bernoulli(params_c, value);
                let database_entry = DatabaseEntry {
                    trace_entry,
                    likelihood: 0.,
                };
                tracedb.insert(name, database_entry);
                value
            }
        };
        value_t
    };
    if x {
        17
    } else {
        29
    }
}

fn main() {
    let mut tracedb = Database::new();

    let r = probfunc(&mut tracedb);
    println!("{}\n{:#?}", r, tracedb);
}
