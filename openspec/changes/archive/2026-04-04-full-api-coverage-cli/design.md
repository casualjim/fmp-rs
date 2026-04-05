## Context

The `fmp` crate is a workspace with two members:
- `fmp` (root) — the API library with 27 endpoint trait modules under `src/endpoints/`
- `fmp-cli` — the CLI crate under `crates/fmp-cli/`

The CLI follows a strict, consistent pattern:
1. Each domain area has a file in `crates/fmp-cli/src/cli/<name>.rs`
2. That file defines a `*Args` enum deriving `clap::Subcommand`, with variants per API function
3. Each variant's inner `*Args` struct derives `clap::Args` with `#[arg(...)]` fields
4. `cli/mod.rs` re-exports the `*Args` type
5. `config.rs` adds a `Commands::<Name>(*Args)` variant to the top-level enum
6. `cli/mod.rs#dispatch()` adds the match arm calling `args.handle(&ctx).await`

Six API modules have no CLI file at all. Several existing CLI files omit some API functions. No API library changes are needed.

## Goals / Non-Goals

**Goals:**
- Add CLI modules for `insider_trades`, `government_trading`, `form_13f`, `fund`, `fundraisers`, and `technical_indicators`
- Fill the partial-coverage gaps in `analyst`, `calendar`, `news`, `sec_filings`, `company`, `crypto`, and `forex`
- Register every new command in `cli/mod.rs`, `config.rs`, and `cli/mod.rs#dispatch()`
- Follow the existing clap pattern exactly — no new frameworks, no new output formats

**Non-Goals:**
- Changes to the API library (`src/`)
- New output formats (all output remains `output_json`)
- Table/CSV/pager rendering (out of scope)
- Interactive mode or TUI

## Decisions

### D1: One CLI file per API module

**Decision:** Each new API module gets its own `cli/<name>.rs` file, matching the 1:1 convention already used.

**Alternatives considered:**
- Group related small APIs (e.g., insider trades + government trading into one `political.rs`). Rejected: breaks the convention and makes discoverability harder.

### D2: Technical indicators stay in chart AND get a dedicated command

**Decision:** Keep technical indicators in `chart.rs` as they are (no regression), AND add a new `technical-indicators` top-level command that exposes the full `TechnicalIndicatorsApi` directly.

**Alternatives considered:**
- Remove technical indicators from `chart.rs`. Rejected: breaking change.
- Only add the dedicated command, no changes to `chart.rs`. Accepted and equivalent — the dedicated command is the deliverable; `chart.rs` is untouched.

### D3: Partial-coverage gaps filled in-place

**Decision:** Add missing subcommands to existing CLI files (e.g., add `Dividends` variant to `CalendarArgs`). No new files for partial fills.

### D4: No optional parameter builder short-circuits

**Decision:** Follow the existing pattern where optional params use `match self.field { Some(v) => builder.field(v).build(), None => builder.build() }`. Do not introduce a helper or macro for this.

## Risks / Trade-offs

- **Clap command count growth** → The top-level help output will grow. Mitigation: each new command has a clear one-line description.
- **Unstable FMP parameter types** → Some param structs may have required fields not yet known at design time. Mitigation: read the type definitions in `src/types/` before coding each command.
- **`technical-indicators` name conflict** → The existing `chart` command already surfaces technical indicators. Mitigation: document the overlap in the `technical-indicators` command's help text.

## Open Questions

- Should `technical-indicators` be a top-level command or a subcommand of `chart`? Current decision: top-level, to match the API module boundary and be discoverable without knowing about `chart`.
