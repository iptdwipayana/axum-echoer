# Axum Echoer

Simple health and echo API built with Axum Rust.

## Installation

1. Clone the repository
   ```bash
   git clone https://github.com/iptdwipayana/axum-echoer
   ```
2. Run the application
   ```bash
   cargo run
   ```

## Usage

| Endpoint  | What it does?                                  |
| --------- | ---------------------------------------------- |
| `/health` | Return a simple JSON that says `ok`            |
| `/echo`   | Return whatever JSON you post to this endpoint |
