use criterion::{black_box, BenchmarkId, criterion_group, criterion_main, Criterion};
use graphql_parser::query::{parse_query, Document};
use std::collections::HashMap;

use std::hash::BuildHasherDefault;
use fasthash::{Murmur3HasherExt};

mod query;
use query::QUERY;

type Cache = HashMap<
    &'static str,
    Document<'static, &'static str>,
    BuildHasherDefault<Murmur3HasherExt>
>;

pub fn parse(query: &'static str) -> Document<'static, &'static str> {
    parse_query::<&str>(query).unwrap()
}

pub fn cached_parse<'a>(query: &'static str, cache: &'a mut Cache) -> &'a Document<'static, &'static str> {
    cache.entry(query)
       .or_insert_with(|| parse_query::<&str>(query).unwrap())
}

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("GraphQL Parsing");
    let mut cache: Cache = Default::default();
    for i in [20u64].iter() {
        group.bench_with_input(BenchmarkId::new("Cached", i), i, 
            |b, _| b.iter(|| { cached_parse(black_box(QUERY), &mut cache); }));
        group.bench_with_input(BenchmarkId::new("No cache", i), i, 
            |b, _| b.iter(|| { parse(black_box(QUERY)); }));
    }
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);