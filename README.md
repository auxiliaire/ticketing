# Ticketing Application

## A fullstack application example in the Rust ecosystem

![Rust Security Audit](https://github.com/auxiliaire/ticketing/actions/workflows/audit.yml/badge.svg)
![Rust Build](https://github.com/auxiliaire/ticketing/actions/workflows/general.yml/badge.svg)

This is an integrated Rust backend and frontend (with Yew) example.

## Requirements

* Rust nighlty

## Tech Stack

### Infrastructure

* Rust (~100%)
* Redis
* MySQL

### Backend

* Axum
* Tokio
* SeaORM
* Serde

### Frontend

* Yew
* Tailwind CSS
* daisyUI
* Bulma

## Preview

![Preview Image](https://raw.githubusercontent.com/auxiliaire/ticketing/testing/gallery/project-ticket-board-in-progress.jpg)

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

### Or combined

```bash
./start.sh
```

Visit [Localhost](http://127.0.0.1:8080/).
