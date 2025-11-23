# Lbyl Fmp-Rs

## Tests
- `cargo nextest run --all-features` - runs the suite. An FMP API key is required for all tests; set `FMP_API_KEY=<your key>` before running.
- Live tests always hit the real FMP API; they will fail fast if `FMP_API_KEY` is missing.
