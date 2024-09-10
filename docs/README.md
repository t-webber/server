# Server

## Setting up diesel

1. Download `sqlite3-dll-win-x64-*.zip` from [the sqlite download page](https://www.sqlite.org/download.html).
1. Install the diesel CLI with

    ```sh
    cargo install diesel_cli --no-default-features --features sqlite
    ```

1. Extract the contents of the `.zip` file and add the folder to the `PATH`.
1. Add a `.env` file to the root of the project with

    ```sh
    echo 'DATABASE_URL="file:./diesel/dev.db"' > .env
    ```

1. Create the given directory of the database file

    ```sh
    mkdir db
    ```

1. To create the database, run

    ```sh
    diesel setup
    ```

1. To create a migration, run

    ```sh
    diesel migration generate init
    ```

1. Update your `.gitignore` file.
1. To create a migration, run

    ```sh
    diesel migration generate init
    ```

1. Two files were generated, `up.sql` and `down.sql`. Fill them with

    ```sql
    -- up.sql
    CREATE TABLE posts (
      id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
      title VARCHAR NOT NULL,
      body TEXT NOT NULL,
      published BOOLEAN NOT NULL DEFAULT 0
    )
    ```

    ```sql
    -- down.sql
    DROP TABLE posts
    ```

1. To apply the migration, run

    ```sh
    diesel migration run
    ```

1. If the `down.sql`, it allows you to redo the migration with

    ```sh
    diesel migration redo
    ```

1. You can know define and run the migrations from 🦀! See [docs](https://diesel.rs/guides/getting-started) for more details.
