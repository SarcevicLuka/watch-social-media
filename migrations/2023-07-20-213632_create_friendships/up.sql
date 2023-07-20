CREATE TABLE friendships
(
    id              VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    user_requesting VARCHAR(36) NOT NULL,
    user_responding VARCHAR(36) NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_friendships_id PRIMARY KEY (id),
    CONSTRAINT friendship_has_one_user_requesting
        FOREIGN KEY (user_requesting)
            REFERENCES users(id)
                ON DELETE CASCADE
                ON UPDATE NO ACTION,
    CONSTRAINT friendship_has_one_user_responding
        FOREIGN KEY (user_responding)
            REFERENCES users(id)
                ON DELETE CASCADE
                ON UPDATE NO ACTION
);

SELECT diesel_manage_updated_at('friendships');