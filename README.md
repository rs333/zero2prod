[![Rust](https://github.com/rs333/zero2prod/actions/workflows/general.yml/badge.svg?branch=main)](https://github.com/rs333/zero2prod/actions/workflows/general.yml)
[![Security audit](https://github.com/rs333/zero2prod/actions/workflows/audit.yml/badge.svg?branch=main)](https://github.com/rs333/zero2prod/actions/workflows/audit.yml)

# Zero to Production in Rust
## How to build
```bash
cargo build
```
## How to test
Veriy the postgres database is running, then run the tests.
```bash
scripts/init_db.sh
cargo test
```
## Notes
How to remove the test databases using psql
```bash
for dbname in $(psql -U postgres -c "copy (select datname from pg_database where datname like '%-%-%-%-%') to stdout"); do     echo "$dbname"; psql -U postgres -c "drop database \"$dbname\""; done
```