
FROM timescale/timescaledb:latest-pg15


COPY init.sql /docker-entrypoint-initdb.d/00-init.sql
COPY schema.sql /docker-entrypoint-initdb.d/01-schema.sql
