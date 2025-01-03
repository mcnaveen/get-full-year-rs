
# **get-full-year 🦀**

[![Crates.io](https://img.shields.io/crates/v/get-full-year.svg)](https://crates.io/crates/get-full-year)  
[![Docs.rs](https://docs.rs/get-full-year/badge.svg)](https://docs.rs/get-full-year)  
[![License: MIT or Apache-2.0](https://img.shields.io/crates/l/get-full-year)](https://opensource.org/licenses/MIT)

An unofficial Rust client for [getfullyear.com](https://getfullyear.com) that helps you get the full year. Because sometimes you just need to know what year it is, and you need to know it _properly_.

---

## 🚀 Features

- Written in Rust with type safety
- Async API powered by `reqwest` and `tokio`
- Clean and simple design
- Handles errors gracefully
- Includes comprehensive documentation

---

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
get-full-year = "1.0.0"
```

## 🔨 Usage

### Basic Example

```rust
use get_full_year::get_full_year;

#[tokio::main]
async fn main() {
    match get_full_year(false).await {
        Ok(data) => println!("Fetched Year Data: {:?}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```
### Enterprise Mode
```rust
use get_full_year_rs::get_full_year;

#[tokio::main]
async fn main() {
    let is_enterprise = true; // Enterprise mode
    match get_full_year(is_enterprise).await {
        Ok(data) => println!("Enterprise Data: {:?}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## 🔍 API Reference
`get_full_year(is_enterprise: bool) -> Result<YearResponseDTO, Box<dyn Error>>`

Makes a sophisticated request to [getfullyear.com](https://getfullyear.com) to fetch the current year data.

Parameters:
- `is_enterprise` (required): bool
  - Enables enterprise mode, suppressing sponsorship messages.

Returns:
- `Ok(YearResponseDTO)`: The current year data.
- `Err(Box<dyn Error>)`: If an error occurs during the request.

### Example

```rust
let data = get_full_year(false).await?;
println!("Year: {}", data.year);
```

### Data Structure

- YearResponseDTO
```rust
pub struct YearResponseDTO {
    pub year: u32,
    pub sponsored_by: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}
```
- YearFetchingError

A custom error type that encapsulates errors occurring during the year-fetching process.

```rust
pub enum YearFetchingError {
    RequestError(reqwest::Error),
    JsonParsingError(serde_json::Error),
}
```

## 🏗️ Project Structure
```
get-full-year-rs/
├── src/           # Source code
│   ├── dto.rs      # Data structures
│   ├── errors.rs   # Error handling
│   ├── services.rs # Business logic
│   └── lib.rs      # Library entry point
├── Cargo.toml     # Crate metadata
└── README.md      # Documentation
```

## 🛠️ Development

Clone the repository and run the following command to install dependencies:

```
git clone https://github.com/mcnaveen/get-full-year-rs.git
cd get-full-year-rs
```

Build the project:

```bash
cargo build
```

Run the tests:

```bash
cargo test
```

Run the project:

```bash
cargo run
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👤 Author

- Name: Naveen MC
- Email: me@mcnaveen.com
- GitHub: @mcnaveen

## 🙏 Acknowledgments

- Thanks to [getfullyear.com](https://getfullyear.com) for providing this essential service
- The Rust community for making Rust awesome
- The open-source community for continuous inspiration


## 📊 Stats

![GitHub stars](https://img.shields.io/github/stars/mcnaveen/get-full-year-rs?style=social)
![crates.io](https://img.shields.io/crates/v/get-full-year?style=social)

---

<sub>Made with ❤️ and probably not coffee ☕️</sub>