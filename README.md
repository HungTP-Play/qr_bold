# QR Bold

QR Bold is a simple tool and library to generate QR codes.

## Installation

```bash
$ go get github.com/qr-bold/qr
```

## Usage

```rust
// This code is just for template the doc, it's not runnable.
// Document will be update later.
use qr::Qr;

fn main() {
    let qr = Qr::new("Hello, world!").unwrap();
    println!("{}", qr);
}
```