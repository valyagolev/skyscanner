# skyscanner

[![Crates.io](https://img.shields.io/crates/v/skyscanner.svg)](https://crates.io/crates/skyscanner)
[![Docs.rs](https://docs.rs/skyscanner/badge.svg)](https://docs.rs/skyscanner)

Currently, implementation of the Skyscanner Flights live pricing API (3.0).

https://developers.skyscanner.net/api/flights-live-pricing#tag/FlightsService/operation/FlightsService_CreateSearch

Bare-bones, but mostly well-typed. Useful for me. Feel free to contribute/fork/ask for things.

Almost no docs, but simple to use:

set `SKYSCANNER_API_KEY` env var

```rust
    let req = CreateRequest {
        query_legs: vec![QueryLeg {
            origin_place_id: "LAX".into(),
            destination_place_id: "BER".into(),
            date: Local::now().date_naive() + Duration::days(2),
        }],
        ..Default::default()
    };

    let mut str = pin!(stream_search(req));

    while let Some(resp) = str.next().await {
        let content = resp.unwrap();

        for itin in content.sorted(resp::LiveSortingOption::Cheapest).take(5) {
            println!("{}", content.format_itinerary(&itin));
        }

        println!("====more?...");
    }
```
