# CONFIG

## TEST COVERAGE

```bash
# Create files to check coverage
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='target/coverage/cargo-test-%p-%m.profraw' cargo test

# Create html test coverage
grcov target/coverage --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/html

# Create Lcov test coverage
grcov target/coverage --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o coverage/tests.lcov

# Script to check if the coverage is ok
./check_coverage.sh
```

the coverage on html is available in router [:host]/coverage, exemple (http://localhost:8080/coverage)

## RUN LINT

```bash
cargo clippy -- -D warnings
```

## CLEAN SCRIPT

```sql
DELETE FROM public.emails;
DELETE FROM public.telephones;
DELETE FROM public.profiles;
DELETE FROM public.users;
DELETE FROM public.sign_ins;
```

## EXTENSIONS

coverage gutters
Even Better TOML
rust-analyzer
gitLens
SQLTools
SQLTools PostgresSQL
Trunk Check
Multiple cursor case preserve
Inline SQL

## ADD MIGRATIONS

```bash
#add migration up and drop
sqlx migrate add -r <name> --source ./database/migration
```

## RUN MIGRATIONS

```bash
sqlx migrate run --source ./database/migration
```

## REVERT MIGRATION

```bash
sqlx migrate revert --source ./database/migration
```
