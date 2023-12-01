#!/usr/bin/env bash

set -x
set -eo pipefail


if ! [[ -x "$(which psql)" ]]; then
  echo >&2 "psql is needed to run init_db"
  exit 1
fi

if ! [[ -x "$(which sqlx)" ]]; then
  echo >&2 "sqlx not found"
  echo >&2 "Use: "
  echo >&2 "cargo install --version='~0.7' sqlx-cli --no-default-features --features rustls,postgres"
  echo >&2 "to install it."
  exit 1
fi

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=mailer}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"

if [[ -z "${SKIP_DOCKER}" ]]; then
  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
fi

until PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is not available yet. Sleeping..."
  sleep 1
done

>&2 echo "Postgres is up and running in port ${DB_PORT}!"

>&2 echo "Running migrations"
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated!"
