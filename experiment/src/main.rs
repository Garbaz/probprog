use experiment::new_structure2::{FnProb, Sample};
use experiment::primitive::{uniform, bernoulli};
use experiment::{
    new_structure2::Distribution,
    trace::{Trace, TraceDirectory},
};

pub fn t4(p: f64) -> impl FnProb<usize> {
    move |__trace: &mut Trace| {
        let mut __log_likelihood = 0.;
        let value = (|| {
            let _q = {
                let s = uniform(-1., 1.).resample(__trace);
                __log_likelihood += s.log_likelihood;
                s.value
            };

            let mut c = 0;
            {
                let mut __loop_counter: usize = 0;
                loop {
                    let __trace =
                        __trace.descend(TraceDirectory::Loop(__loop_counter));

                    {
                        let x = {
                            let s = bernoulli(p).resample(__trace);
                            __log_likelihood += s.log_likelihood;
                            s.value
                        };

                        if x {
                            return c;
                        } else {
                            c += 1;
                        }
                    }

                    __loop_counter += 1;
                }
            }
        })();
        Sample {
            value,
            log_likelihood: __log_likelihood,
        }
    }
}

fn main() {
    let pf = t4(0.33);
    println!("{:#?}", pf.sample());
}
