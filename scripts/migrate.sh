#!/usr/bin/env bash

set -x
set -eo pipefail
if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 " cargo install --version='~0.8' sqlx-cli \
--no-default-features --features rustls,postgres"
  echo >&2 "to install it."
  exit 1
fi


export DATABASE_URL=postgres://$PG_SUPERUSER:$PG_SUPERUSER_PWD@127.0.0.1:5432/$CONFIG_DB_NAME

#the migrations table is in the public schema, so perms a little messy atm
sqlx database create
sqlx migrate run