#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
	CREATE USER assessment_user WITH PASSWORD 'test_password';
	CREATE DATABASE assessment;
	GRANT ALL PRIVILEGES ON DATABASE assessment TO assessment_user;
EOSQL