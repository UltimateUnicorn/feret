# FERET : Furuno ECDIS Route Export Tool

FERET is a tool to convert [Furuno](https://www.furuno.com/en/)'s [ECDIS](https://en.wikipedia.org/wiki/Electronic_navigational_chart) route files (*.rtz) into CSV files. I then copy the data from the later ones into an Excel file that my colleagues and I use to perform some nautical calculation. I develop FERET as a practical way to learn Rust.

## Installation

Clone the repository
```bash
git clone https://github.com/UltimateUnicorn/feret.git
```

## Usage

You need a Rust environnement . The input file has to be named "route.rtz" and be in the same directory as the executable. FERET will output a route.csv file upon execution.
```bash
cargo run
```

## License

[Apache-2.0](https://github.com/UltimateUnicorn/feret/blob/main/LICENSE)
