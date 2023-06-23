# Ticketing Application

## A fullstack application example in the Rust ecosystem

![Rust Security Audit](https://github.com/auxiliaire/ticketing/actions/workflows/audit.yml/badge.svg)
![Rust Build](https://github.com/auxiliaire/ticketing/actions/workflows/general.yml/badge.svg)
![JavaScript Build](https://github.com/auxiliaire/ticketing/actions/workflows/npm.yml/badge.svg)

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

```
rustup override set nightly
touch dev.sqlite
```