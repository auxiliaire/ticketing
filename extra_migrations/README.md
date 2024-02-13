# Extra Migrations

These migrations target another database besides the default.

They use their own environment variables and CLI.

They are not run automatically on startup, but have to be prepared in advance.

## Instructions

This section documents what needs to be done to run the migrations in this directory.

### Fang

To run the migrations required by Fang, follow these instructions:

1. Make sure Diesel CLI is installed, or install it using:

    `cargo install diesel_cli --no-default-features --features "postgres sqlite mysql"`

2. Run the migrations from the command line at project root (provided POSTGRES_URL env var is set):

    ˙diesel migration run --database-url $POSTGRES_URL --migration-dir ./extra_migrations/fang`
