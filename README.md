# GraphQL caching parsed query benchmark
Since some queries in GraphQL can reach large size it might be optimal to save the result into `HashMap<QueryString, GraphQLDocument>`. This benchmark tries to give an objective analysis of whether it is plausable. \
Rust is selected because it is performant with relatively predictable results (unlike VM-backed languages)

## Results
CPU: i7-8550U CPU @ 1.80GHz \
RAM: 16GB 2400 MHz \
Benchmark tool: [criterion](https://docs.rs/criterion/0.3.4/criterion/)

### 4 kilobytes query
|Variant|Time|
|-|-|
|Cached|942.86 ns 957.41 ns 977.26 ns|
|No Cache|158.22 μs 159.75 μs 161.40 μs|

The difference between cached and non-cached variants is about 166 times


### 41 bytes query
|Variant|Time|
|-|-|
|Cached|42.394 ns 43.069 ns 43.911 ns|
|No Cache|4.6407 μs 5.0015 μs 5.4416 μs|

The difference between cached and non-cached variants is about 116 times

## Running the benchmark
Make sure you have `cargo` and `rustc` installed
- Clone the repo
- Execute `cargo bench`
