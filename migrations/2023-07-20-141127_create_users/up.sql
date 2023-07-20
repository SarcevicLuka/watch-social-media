CREATE TABLE users
(
    id         VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    email      VARCHAR(255) NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    last_name  VARCHAR(255) NOT NULL,
    nick_name  VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    avatar     TEXT DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_users_id PRIMARY KEY (id)
);

SELECT diesel_manage_updated_at('users');

CREATE INDEX IF NOT EXISTS users_email        ON users USING BTREE(email);
CREATE INDEX IF NOT EXISTS users_first_name   ON users USING BTREE(first_name);
CREATE INDEX IF NOT EXISTS users_last_name    ON users USING BTREE(last_name);
CREATE INDEX IF NOT EXISTS users_nick_name    ON users USING BTREE(nick_name);
CREATE INDEX IF NOT EXISTS users_nick_avatar  ON users USING BTREE(avatar);