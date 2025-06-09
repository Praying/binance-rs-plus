use criterion::{criterion_group, criterion_main, Criterion};
use binance_rs_plus::model::DayTickerEvent; // For deserializing ticker events
use core::time::Duration;

// main function for the benchmark will be async
async fn criterion_benchmark_async(c: &mut Criterion) {
    let mut group = c.benchmark_group("websockets-json-deserialization");

    // Fetch data asynchronously once
    let client = reqwest::Client::new();
    let all_symbols_json = client
        .get("https://api.binance.com/api/v3/ticker/24hr") // !ticker@arr is for websockets, REST equivalent is /api/v3/ticker/24hr for all symbols
        .send()
        .await
        .expect("Failed to fetch all symbols for benchmark")
        .text()
        .await
        .expect("Failed to get text for all symbols");

    let btc_symbol_json = client
        .get("https://api.binance.com/api/v3/ticker/24hr?symbol=BTCUSDT") // <symbol>@ticker equivalent for REST
        .send()
        .await
        .expect("Failed to fetch BTCUSDT for benchmark")
        .text()
        .await
        .expect("Failed to get text for BTCUSDT");

    group.sample_size(200); // Adjust as needed
    group.measurement_time(Duration::new(10, 0)); // Shorter time for faster local benchmarks

    // Benchmark deserializing the JSON for all symbols (which is an array of DayTickerEvent-like structures)
    group.bench_function("deserialize_all_symbols_ticker_arr", |b| {
        b.iter(|| {
            let _events: Vec<serde_json::Value> = serde_json::from_str(&all_symbols_json).unwrap();
            // If DayTickerEvent matches the structure of elements in the /api/v3/ticker/24hr response array:
            // let _events: Vec<DayTickerEvent> = serde_json::from_str(&all_symbols_json).unwrap();
            // For now, using serde_json::Value to avoid potential mismatch, focusing on raw parse.
        });
    });

    // Benchmark deserializing the JSON for a single symbol ticker
    group.bench_function("deserialize_single_symbol_ticker", |b| {
        b.iter(|| {
            // The /api/v3/ticker/24hr?symbol=BTCUSDT returns a single object, not an array.
            // This should map to DayTickerEvent if the fields are compatible.
            let _event: DayTickerEvent = serde_json::from_str(&btc_symbol_json).unwrap();
        });
    });

    group.finish();
}

// Criterion setup for async benchmark
fn benchmark_wrapper(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(criterion_benchmark_async(c));
}

criterion_group!(benches, benchmark_wrapper);
criterion_main!(benches);
