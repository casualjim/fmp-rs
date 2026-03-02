## Context

The fmp-rs library provides type-safe, async Rust bindings for the Financial Modeling Prep (FMP) API, covering 20+ endpoint categories including quotes, charts, company data, crypto, forex, and advanced analytics. The library is well-structured with:

- A central `FmpHttpClient` that implements all API traits
- Separate trait-based modules for each API category
- Comprehensive type definitions with serde support
- Built-in retry logic, tracing, and middleware support

Currently, users must write Rust code to interact with the API. A CLI would provide immediate, scriptable access to financial data without requiring compilation or coding.

**Constraints:**
- Must reuse existing fmp library without modifications
- Should follow Unix CLI conventions (stdin/stdout, exit codes, composability)
- Configuration should be secure (API key handling) and portable across OSes
- Performance overhead should be minimal compared to direct library usage

## Goals / Non-Goals

**Goals:**
- Provide comprehensive CLI coverage of all fmp library endpoints
- Implement intuitive namespaced command structure matching API categories
- Support JSON output format for programmatic processing
- Enable secure API key management via environment variables or config files
- Follow CLI best practices: helpful error messages, progress indicators, shell completion
- Support both interactive exploration and scripting use cases

**Non-Goals:**
- Interactive TUI (terminal user interface) - focus on command-based workflow
- Data caching or offline mode - each command hits the live API
- Data transformation or analysis features - output raw API data
- Authentication beyond API key (no OAuth, session management)
- Windows-specific features (maintain cross-platform compatibility)

## Decisions

### 1. Workspace Structure: Separate Binary Crate

**Decision:** Create `crates/fmp-cli` as a separate binary crate in the workspace, depending on the `fmp` library crate.

**Rationale:** 
- Clean separation between library and CLI concerns
- Allows independent versioning and release cycles
- Library users don't pay CLI dependency cost
- Matches Rust ecosystem patterns (e.g., `serde`/`serde_json`, `tokio`/`tokio-console`)

**Alternatives considered:**
- Single crate with binary: rejected due to dependency bloat for library users
- Separate repository: rejected due to tight coupling and maintenance overhead

### 2. CLI Framework: clap with Derive API

**Decision:** Use `clap` 4.x with derive macros for command-line argument parsing.

**Rationale:**
- Industry standard, actively maintained, excellent documentation
- Derive API provides type-safe command definitions with minimal boilerplate
- Built-in support for subcommands, which maps perfectly to our namespaced structure
- Automatic shell completion generation (bash, zsh, fish, etc.)
- Rich help text generation and error messages

**Alternatives considered:**
- `argh`: lighter weight but fewer features, less ecosystem support
- `structopt`: older, superseded by clap derive API
- Manual parsing: error-prone, reinventing the wheel

### 3. Command Structure: Hierarchical Namespaces

**Decision:** Use hierarchical subcommands mapping to API categories: `fmp <namespace> <action> [args]`

**Examples:**
```bash
fmp quotes get AAPL MSFT GOOGL
fmp chart historical AAPL --from 2024-01-01 --to 2024-12-31 --interval 1h
fmp crypto price BTCUSD ETHUSD
fmp company profile AAPL
fmp market search "apple" --limit 10
```

**Rationale:**
- Matches user mental model of API categories
- Scales well as new endpoints are added
- Enables namespace-level help (e.g., `fmp quotes --help`)
- Follows patterns from kubectl, docker, aws CLI

**Alternatives considered:**
- Flat command structure (`fmp-get-quote`, `fmp-get-chart`): rejected due to poor discoverability
- Single command with flags: rejected due to complexity and poor UX

### 4. Configuration: Clap with Environment and File Support

**Decision:** Use `clap` for all CLI argument parsing, with environment variable and config file support.

**Architecture:**
```rust
#[derive(clap::Parser, Debug, Clone)]
pub struct Cli {
  #[arg(long = "base-url", env = "FMP_BASE_URL", 
        default_value = "https://financialmodelingprep.com/api/v3/")]
  pub base_url: String,

  #[arg(long = "api-key", env = "FMP_API_KEY")]
  pub api_key: Option<SecretString>,

  #[arg(long = "config-file")]
  pub config_file: Option<PathBuf>,
}
```

**Priority order:**
1. Command-line flags (highest priority)
2. Environment variables
3. Config file values
4. Built-in defaults (lowest priority)

**Configuration loading:**
```rust
pub(crate) fn load_config(cli: &Cli) -> Result<FmpConfig> {
  let mut config = FmpConfig::from_cli(cli);
  
  if let Some(path) = &cli.config_file {
    let file_config: FmpConfig = toml::from_str(&fs::read_to_string(path)?)?;
    config.merge(file_config);
  }
  
  let cfg_dir = dirs::home_dir()
    .ok_or_else(|| eyre::eyre!("Could not determine home directory"))?
    .join(".config/fmp-cli/config.toml");
  
  if cfg_dir.exists() {
    let file_config: FmpConfig = toml::from_str(&fs::read_to_string(&cfg_dir)?)?;
    config.merge(file_config);
  }

  Ok(config)
}
```

**Rationale:**
- Clap provides built-in environment variable support via `env` attribute
- Simple merge logic for layered configuration
- dirs crate handles cross-platform paths correctly
- toml provides human-readable config files

**Alternatives considered:**
- Manual config parsing: rejected due to boilerplate
- Only environment variables: rejected due to lack of persistence for settings

### 5. Output Formatting: JSON Only

**Decision:** Output all data as JSON to stdout, enabling composition with tools like `jq`.

**Rationale:**
- JSON is universally supported and well-understood
- Enables piping to `jq` for filtering and transformation
- Simpler implementation without multiple formatters
- Users can transform JSON to other formats with external tools

**Alternatives considered:**
- Multiple output formats: rejected to keep CLI simple and focused
- Table output: rejected as `jq` can format tabular data when needed

### 6. Error Handling: User-Friendly Messages + Exit Codes

**Decision:** Display actionable error messages with context, use appropriate exit codes (1 for errors, 0 for success).

**Example:**
```
Error: Failed to fetch quote for INVALID

Caused by:
  API returned 404: Symbol not found

Hint: Verify the symbol exists using 'fmp market search <query>'
```

**Rationale:**
- Follows CLI best practices (rustc, cargo style)
- Exit codes enable scripting (if/fi in bash, etc.)
- Hints guide users toward resolution

### 7. Async Runtime: tokio

**Decision:** Use tokio as the async runtime for CLI execution.

**Rationale:**
- Already used by fmp library via reqwest
- Industry standard, excellent performance
- `#[tokio::main]` simplifies CLI entry point

**Alternatives considered:**
- async-std: rejected due to library already using tokio
- smol: rejected due to smaller ecosystem

## Risks / Trade-offs

**Risk: API key security** → Mitigation: Use secrecy::SecretString for API key in config, store in config file with restrictive permissions (0600), support environment variables for CI/CD, never log or display API keys in output, zeroize on drop

**Risk: Large output overwhelming terminal** → Mitigation: Add `--limit` flags, pipe to pager (e.g., `less`) automatically for long output when TTY detected

**Risk: Network errors disrupting scripts** → Mitigation: Retry logic already in fmp library, add `--timeout` and `--retries` CLI flags, clear error messages with exit codes

**Risk: Maintenance burden as API evolves** → Mitigation: Code generation for command structs from API traits (future), comprehensive test coverage, keep CLI thin (delegate to library)

**Trade-off: Binary size vs features** → Accept slightly larger binary (~5-10MB) for comprehensive feature set; users who need minimal footprint can use the library directly

**Trade-off: Performance vs ergonomics** → Accept minimal overhead from CLI parsing and formatting (typically <10ms) for significantly better UX

## Migration Plan

This is a new addition, not a migration. Deployment steps:

1. **Initial Release (v0.1.0)**
   - Implement core infrastructure (config, output formatting, error handling)
   - Cover most commonly used endpoints (quotes, chart, company, crypto, forex)
   - Publish to crates.io
   - Add installation instructions to README

2. **Feature Complete (v0.2.0)**
   - Add remaining endpoint categories (commodities, advanced features)
   - Implement shell completion generation
   - Add comprehensive examples and documentation

3. **Future Enhancements**
   - Profile and optimize performance
   - Add caching layer if requested by users
   - Explore interactive mode if demand exists

**Rollback:** Not applicable - this is additive, no existing functionality is being replaced.

## Open Questions

1. **Shell completion installation:** Should we provide automatic installation scripts for bash/zsh/fish, or rely on manual setup?

2. **Streaming output:** For large datasets (e.g., historical data spanning years), should we stream JSON arrays line-by-line or buffer entire response?

3. **Interactive configuration:** Should first-run experience include interactive setup wizard for API key?

4. **Version checking:** Should CLI check for updates and notify users of newer versions?

These questions can be resolved during implementation based on user feedback and practical constraints.
