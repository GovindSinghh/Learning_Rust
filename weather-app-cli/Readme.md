# Weather App CLI 🌤️

A fast and beautiful command-line weather application built in Rust that provides current weather information for any city around the world.

## Features ✨

- **Real-time Weather Data**: Get current weather conditions using the OpenWeatherMap API
- **Metric Units**: All temperatures and measurements in metric units (Celsius, m/s)
- **Fast Performance**: Built with Rust for optimal performance

## Installation 🚀

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Build from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd weather-app-cli
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

## Usage 📖

### Get Current Weather

To get the current weather for a specific city:

```bash
cargo run -- current-weather --city "London"
```

Or using the short flag:

```bash
cargo run -- current-weather -c "New York"
```

### Example Output

```
🌥️  Weather Summary of London
Temp    feels    Humidity    Wind-Speed
15.2    13.8     67          4.1
```

## Available Commands

| Command | Description | Flags |
|---------|-------------|-------|
| `current-weather` | Get current weather for a city | `-c, --city <CITY_NAME>` |

## Technical Details 🔧

### Dependencies

- **clap**: Command-line argument parsing with derive macros
- **reqwest**: HTTP client for API requests
- **serde_json**: JSON serialization/deserialization
- **tokio**: Async runtime for non-blocking operations
- **owo-colors**: Terminal colorization library

### API Integration

The application uses the OpenWeatherMap API to fetch weather data. The API provides:
- Current temperature (in Celsius)
- Feels like temperature
- Humidity percentage
- Wind speed (in m/s)

### Error Handling

The application includes basic error handling for:
- Invalid URLs
- Network request failures
- JSON parsing errors

## Development 🛠️

### Project Structure

```
weather-app-cli/
├── src/
│   └── main.rs          # Main application logic
├── Cargo.toml           # Project dependencies and metadata
├── Cargo.lock           # Locked dependency versions
└── README.md           # This file
```

### Building for Development

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments 🙏

- [OpenWeatherMap](https://openweathermap.org/) for providing the weather API
- The Rust community for the excellent ecosystem of libraries and Rust book.