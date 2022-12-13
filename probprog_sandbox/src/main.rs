use probprog::{
    bernoulli::{Bernoulli, BernoulliParams},
    distribution::Distribution,
    trace::{Database, DatabaseEntry, TraceConfig, TraceEntry},
};

/// A mock-up of how a probabilstic function would end up looking like
/// after being transformed by the macro.
/// Note: We should extract as much as possible from the function itself
/// into pre-written functions, such that the macro shenanigans are kept
/// at a minimum.
fn probfunc(trace: &mut TraceConfig) -> u8 {
    {
        /* PROB MACRO CODE */
        trace.path += "probfunc/";
    }

    let x = {
        /* PROB MACRO CODE (Replaced `bernoulli(0.5)`) */
        let params = BernoulliParams { p: 0.5 };
        let distribution = Bernoulli::new(params).unwrap();
        let name = trace.path.clone() + "x";
        let value_t = match trace.database.get(&name) {
            Some(DatabaseEntry {
                trace_entry: TraceEntry::Bernoulli(ps, current_value),
                likelihood,
            }) if *ps == params => {
                let proposal = distribution.propose(*current_value);
                let proposal_likelihood = distribution.proposal_likelihood(*current_value, proposal);
                let inverse_likelihood = distribution.proposal_likelihood(proposal, *current_value);
                let score = (inverse_likelihood/proposal_likelihood).min(1.);
                //^ TODO!

                todo!()
            },
            _ => {
                let value = distribution.sample();
                let trace_entry = TraceEntry::Bernoulli(params, value);
                let database_entry = DatabaseEntry {
                    trace_entry,
                    likelihood: distribution.likelihood(value),
                };
                trace.database.insert(name, database_entry);
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
    let mut trace = TraceConfig::new();

    let r = probfunc(&mut trace);
    println!("{}\n{:#?}", r, trace);
}
