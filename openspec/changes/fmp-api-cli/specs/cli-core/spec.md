## ADDED Requirements

### Requirement: Layered configuration with clap
The CLI SHALL use clap for argument parsing with environment variable and config file support.

#### Scenario: Config struct with clap
- **WHEN** defining FmpConfig struct
- **THEN** system uses clap::Parser derive with env attributes for environment variable support

#### Scenario: Configuration priority order
- **WHEN** loading configuration
- **THEN** system applies layers in order: CLI args → env vars → project file → user file → defaults

#### Scenario: CLI flag overrides env var
- **WHEN** both FMP_API_KEY env var and --api-key flag are provided
- **THEN** system uses the --api-key flag value

#### Scenario: Env var overrides config file
- **WHEN** FMP_BASE_URL env var is set and config file also contains base_url
- **THEN** system uses the environment variable value

### Requirement: Cross-platform directory paths
The CLI SHALL use the dirs crate for OS-appropriate configuration and data directories.

#### Scenario: Config file location on Linux
- **WHEN** running on Linux
- **THEN** config file is stored at ~/.config/fmp-cli/config.toml

#### Scenario: Config file location on macOS
- **WHEN** running on macOS
- **THEN** config file is stored at ~/Library/Application Support/fmp-cli/config.toml

#### Scenario: Config file location on Windows
- **WHEN** running on Windows
- **THEN** config file is stored at %APPDATA%\fmp-cli\config.toml

#### Scenario: Project-local config
- **WHEN** .fmp.toml exists in current directory
- **THEN** system loads it as project-specific configuration layer

### Requirement: Secure API key handling
The CLI SHALL securely manage API keys using secrecy crate.

#### Scenario: API key stored as SecretString
- **WHEN** API key is loaded from any source
- **THEN** system wraps it in secrecy::SecretString to prevent accidental exposure

#### Scenario: API key never logged
- **WHEN** debug logging is enabled
- **THEN** API key is never displayed in logs (secrecy prevents Debug/Display)

#### Scenario: API key zeroized on drop
- **WHEN** config object is dropped
- **THEN** API key memory is securely zeroized

### Requirement: API key resolution
The CLI SHALL support multiple methods for providing the FMP API key with defined priority.

#### Scenario: Use command-line flag
- **WHEN** user runs `fmp --api-key YOUR_KEY quotes get AAPL`
- **THEN** system uses the provided API key for the request

#### Scenario: Use environment variable
- **WHEN** FMP_API_KEY environment variable is set and no --api-key flag provided
- **THEN** system uses the environment variable value as API key

#### Scenario: Use config file
- **WHEN** config file contains api_key and no override provided
- **THEN** system uses the config file value as API key

#### Scenario: Missing API key displays error
- **WHEN** no API key is provided via any method
- **THEN** system displays error message explaining how to configure API key

### Requirement: Command-line argument parsing with clap
The CLI SHALL parse command-line arguments using clap derive API with hierarchical subcommands.

#### Scenario: Parse namespaced command
- **WHEN** user runs `fmp quotes get AAPL`
- **THEN** system parses namespace "quotes", action "get", and symbol "AAPL"

#### Scenario: Display help for namespace
- **WHEN** user runs `fmp quotes --help`
- **THEN** system displays all available actions under the quotes namespace

#### Scenario: Invalid command displays error
- **WHEN** user runs `fmp invalid-namespace get AAPL`
- **THEN** system displays error message listing valid namespaces

### Requirement: JSON output format
The CLI SHALL output all data as JSON to stdout.

#### Scenario: JSON output
- **WHEN** user runs `fmp quotes get AAPL`
- **THEN** system outputs data as formatted JSON to stdout

#### Scenario: JSON for piping
- **WHEN** user pipes output to another command
- **THEN** JSON output enables composition with tools like jq

### Requirement: Error handling with eyre
The CLI SHALL provide actionable error messages using eyre with context chains and appropriate exit codes.

#### Scenario: Successful command exits with 0
- **WHEN** command completes successfully
- **THEN** process exits with code 0

#### Scenario: API error exits with 1
- **WHEN** API returns an error (e.g., 404, 500)
- **THEN** process exits with code 1 and displays error with context chain

#### Scenario: Invalid arguments exits with 2
- **WHEN** user provides invalid command-line arguments
- **THEN** process exits with code 2 and displays usage information

#### Scenario: Error message includes context chain
- **WHEN** error occurs with multiple causes
- **THEN** error display shows full chain with indented sources

#### Scenario: Error message includes hint
- **WHEN** symbol not found error occurs
- **THEN** error message includes hint suggesting use of search command

### Requirement: Config file management
The CLI SHALL manage configuration file persistence and updates.

#### Scenario: Save config to file
- **WHEN** user runs command that updates config (e.g., setting default format)
- **THEN** system saves updated config to user config directory

#### Scenario: Config file permissions
- **WHEN** config file is created
- **THEN** file permissions are set to 0600 (owner read/write only) to protect API key

#### Scenario: Create config directory if missing
- **WHEN** saving config and directory doesn't exist
- **THEN** system creates parent directories recursively

### Requirement: Version and help information
The CLI SHALL display version information and comprehensive help text.

#### Scenario: Display version
- **WHEN** user runs `fmp --version`
- **THEN** system displays version number in format "fmp-cli x.y.z"

#### Scenario: Display global help
- **WHEN** user runs `fmp --help`
- **THEN** system displays available namespaces, global flags, and usage examples

#### Scenario: Display command-specific help
- **WHEN** user runs `fmp quotes get --help`
- **THEN** system displays specific help for the quotes get command including all flags and arguments

### Requirement: Colored output with owo-colors
The CLI SHALL provide colored output for errors and highlights when TTY detected.

#### Scenario: Colored error messages
- **WHEN** error is displayed and stderr is TTY
- **THEN** error text is colored red with ❌ emoji

#### Scenario: Context chain styling
- **WHEN** error chain is displayed
- **THEN** source errors are displayed in bright black with bullet points

#### Scenario: Disable colors when piped
- **WHEN** output is piped (not TTY)
- **THEN** colors are automatically disabled
