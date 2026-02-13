# A CLI Temperature Conversion Tool 

A simple CLI tool to convert temperatures between Celsius and Fahrenheit.

## Installation

To use this tool globally as the `temp-conv` command, you need to have [Rust](https://www.rust-lang.org/) installed.

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd temp-conv-tool
   ```

2. Install the binary:
   ```bash
   cargo install --path .
   ```

## Usage

```bash
temp-conv <value> <unit (C/F)>
```

### Examples

- Convert 100°C to Fahrenheit:
  ```bash
  temp-conv 100 C
  ```
  Output: `100.00°C is 212.00°F`

- Convert 32°F to Celsius:
  ```bash
  temp-conv 32 F
  ```
  Output: `32.00°F is 0.00°C`
