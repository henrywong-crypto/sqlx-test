```sh
export DATABASE_URL="sqlite:test.db"
cargo sqlx db create
cargo sqlx migrate run
```
