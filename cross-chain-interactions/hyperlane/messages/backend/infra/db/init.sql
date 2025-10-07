
-- Create database and user for the API
CREATE DATABASE crosschain;

\connect crosschain;

CREATE USER postgres WITH PASSWORD 'postgres';
GRANT ALL PRIVILEGES ON DATABASE crosschain TO postgres;
