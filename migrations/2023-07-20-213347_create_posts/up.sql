CREATE TABLE posts
(
    id         VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    user_id    VARCHAR(36) NOT NULL,
    watch_id   VARCHAR(255) NOT NULL,
    review     TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_posts_id PRIMARY KEY (id),
    CONSTRAINT post_has_one_user
        FOREIGN KEY (user_id)
            REFERENCES users(id)
                ON DELETE CASCADE
                ON UPDATE NO ACTION,
    CONSTRAINT post_has_one_watch
        FOREIGN KEY (watch_id)
            REFERENCES watches(id)
                ON DELETE CASCADE
                ON UPDATE NO ACTION
);

SELECT diesel_manage_updated_at('posts');

CREATE INDEX IF NOT EXISTS post_review ON posts USING BTREE(review);