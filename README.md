# Ticketing Application

## A fullstack application example in the Rust ecosystem

![Rust Security Audit](https://github.com/auxiliaire/ticketing/actions/workflows/audit.yml/badge.svg)
![Rust Build](https://github.com/auxiliaire/ticketing/actions/workflows/general.yml/badge.svg)

This is an integrated Rust backend and frontend (with Yew) example.

## Requirements

* Rust nighlty

## Tech Stack

### Infrastructure

* SQLite
* Rust

### Backend

* Axum
* Tokio
* SQLx
* Serde

### Frontend

* Yew
* Tailwind CSS
* daisyUI

## Starting

### Prerequisites

```bash
rustup override set nightly
```

### Start the backend

```bash
cargo -Z unstable-options -C ./ watch -c -w src -x run
```

### Start the frontend

```bash
cd frontend
trunk serve
```

Visit [Localhost](http://127.0.0.1:8080/).
