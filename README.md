| Branch | Status                                                                                                                                                                                                                                                                                                                                                      |
| ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| main   | [![Rust (main)](https://github.com/rs333/zero2prod/actions/workflows/general.yml/badge.svg?branch=main)](https://github.com/rs333/zero2prod/actions/workflows/general.yml) [![Security audit (main)](https://github.com/rs333/zero2prod/actions/workflows/audit.yml/badge.svg?branch=main)](https://github.com/rs333/zero2prod/actions/workflows/audit.yml) |
| dev    | [![Rust (dev)](https://github.com/rs333/zero2prod/actions/workflows/general.yml/badge.svg?branch=dev)](https://github.com/rs333/zero2prod/actions/workflows/general.yml) [![Security audit (dev)](https://github.com/rs333/zero2prod/actions/workflows/audit.yml/badge.svg?branch=dev)](https://github.com/rs333/zero2prod/actions/workflows/audit.yml)     |

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

- How to remove the test databases using psql

  ```bash
  for dbname in $(psql -U postgres -c "copy (select datname from pg_database where datname like '%-%-%-%-%') to stdout"); do     echo "$dbname"; psql -U postgres -c "drop database \"$dbname\""; done
  ```

- Verify the following prior to running tests locally.

  1. The password is updated in the `DATABASE_URL` for the `.env` file.
  2. The database password is updated in the `base.yml` file.
  3. The `sender_email:` field is updated for the `email_client:` portion of the `production.yml` file.
  4. These changes are not commited.
