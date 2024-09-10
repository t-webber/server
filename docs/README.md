# Server

## Run this project

1. Setup `diesel` (see below).
1. Add `PORT` and `HOST` to the `.env.local` file

   ```env
   PORT=8080
   HOST="localhost"
   ```

1. Setup diesel with

   ```sh
   diesel setup
   ```

1. Change `diesel.toml` print_schema path to `src/database/schema.rs`.
1. Run the migrations with

   ```sh
   diesel migration generate --diff-schema init
   ```

1. Run the project with `cargo run`.

## Seting up diesel

> This guide gives you the minimum to run the project. For more information, see [this file](./diesel.md).

1. Download the `sqlite3` precompiled binaries:

   - On Windows: download `sqlite3-dll-win-x64-*.zip` from [the sqlite download page](https://www.sqlite.org/download.html). Extract the contents of the `.zip` file and add the folder to the `PATH`.
   - On Debian/Ubuntu: run `sudo apt install libsqlite3-dev`
   - On Fedora/Centos: run `sudo dnf install sqlite-devel`

1. Install the diesel CLI with

   ```sh
   cargo install diesel_cli --no-default-features --features sqlite
   ```

1. To create the database, run

   ```sh
   diesel setup
   ```

1. Open `diesel.toml` and change the `print_schema` path to `src/database/schema.rs`.

1. To perform a migration with the content of the schema, run

   ```sh
   diesel migration generate --diff-schema init
   ```

1. Update your `.gitignore` file.
