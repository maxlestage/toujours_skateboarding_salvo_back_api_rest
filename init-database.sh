#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES" <<-EOSQL
  CREATE DATABASE toujours_skateboarding_db;
  \c toujours_skateboarding_db
  create extension if not exists "uuid-ossp";
EOSQL