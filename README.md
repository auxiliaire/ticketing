# Ticketing Application

## A fullstack application example in the Rust ecosystem

![Rust Security Audit](https://github.com/auxiliaire/ticketing/actions/workflows/audit.yml/badge.svg)
![Rust Build](https://github.com/auxiliaire/ticketing/actions/workflows/general.yml/badge.svg)
![Docker Build](https://github.com/auxiliaire/ticketing/actions/workflows/docker.yml/badge.svg)

This is an integrated Rust backend and frontend (with Yew) application sample implementation.

## Requirements

* Rust nighlty
* MariaDB
* PostgreSQL
* Mailhog (SMTP)
* Redis

## Tech Stack

### Infrastructure

* Rust (~100%)
* MariaDB
* PostgreSQL
* Redis
* Docker
* Docker Swarm
* Kubernetes

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

![Preview Image](https://raw.githubusercontent.com/auxiliaire/ticketing/master/gallery/project-ticket-board-in-progress.jpg)

![Preview Image](https://raw.githubusercontent.com/auxiliaire/ticketing/master/gallery/project-ticket-board-dark.png)

## Starting

The easiest way to start a development version of the application is using the `start.sh`
script. This will verify the requirements, provide hints if necessary, and start all
components in order, eventually building and running the backend and the frontend as
found in the source code.

Alternatively, a PowerShell script can be used too. This can start the application on
macOS and Windows as well, provided that PowerShell is already installed.
Usage: `pwsh start.ps1`

### Prerequisites

The following prerequisites have to be fulfilled before trying to start the application.
A Docker Compose file is also provided for convenience.

#### Rust Nightly

```bash
rustup override set nightly
```

#### MariaDB

A running MariaDB instance with properly configured user and database as a main database.

#### PostgreSQL

A running PostgreSQL instance with properly configured user and database for scheduling.

#### Mailhog

Mailhog or similar SMTP endpoint to support email sending and 2FA.

#### Redis

A running Redis server for publisher/subscriber functionality.

### Start the backend

```bash
cargo -Z unstable-options -C ./ watch -c -w src -x run
```

### Start the frontend

By default, the backend is listening on [localhost:7000](http://127.0.0.1:8080/),
but it's possible to change that in the .env file, and the frontend has to know about that,
so we pass it as an environment variable.
Similarly, the client uses its own port, which is `7070` by default.

```bash
cd frontend
export SERVER_URL=http://127.0.0.1:7000 && trunk serve --port=7070
```

### Or combined

#### Bash

```bash
./start.sh
```

#### Powershell

Windows:

```powershell
.\start.ps1
```

macOS / Linux:

```powershell
pwsh start.ps1
```

Visit [Localhost](http://127.0.0.1:7070/).
