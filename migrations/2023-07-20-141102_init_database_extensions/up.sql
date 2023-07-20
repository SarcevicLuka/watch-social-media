-- This extension will allow us to use UUID as primary key for objects
-- Set default for every primary key to `uuid_generate_v4()` and it will
-- by default automatically generate UUID so you don’t have to handle it 
-- in your logic. 
-- You are welcome.
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- This exstension will enable use of pgcrypt in the tables
CREATE EXTENSION IF NOT EXISTS "pgcrypto";