# Because rust-analyzer gets confused sometimes...

killall rust-analyzer;
cd probprog_core && cargo clean && cd ..
cd probprog_macro && cargo clean && cd ..
cd probprog_sandbox && cargo clean && cd ..
cargo clean
