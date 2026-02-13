# A CLI Temperature Conversion Tool 

A simple CLI tool to convert temperatures between Celsius and Fahrenheit.

## Installation

To use this tool globally as the `temp-conv` command, you need to have [Rust](https://www.rust-lang.org/) installed.

1. Clone the repository:
   ```bash
   git clone https://github.com/Ray726-I/Temp-Conv-CLI.git
   cd Temp-Conv-CLI
   ```

2. Install the binary:
   ```bash
   cargo install --path .
   ```

## Usage

```bash
temp-conv <value> <unit (c/f)>
```

### Examples

- Convert 100°C to Fahrenheit:
  ```bash
  temp-conv 100 c
  ```
  Output: `100.00°C is 212.00°F`

- Convert 32°F to Celsius:
  ```bash
  temp-conv 32 f
  ```
  Output: `32.00°F is 0.00°C`
