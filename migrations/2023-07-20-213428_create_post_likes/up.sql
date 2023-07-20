CREATE TABLE post_likes
(
    id         VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    user_id    VARCHAR(36) NOT NULL,
    post_id    VARCHAR(36) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_post_likes_id PRIMARY KEY (id),
    CONSTRAINT post_like_has_one_user
        FOREIGN KEY (user_id)
            REFERENCES users(id)
                ON DELETE CASCADE
                ON UPDATE NO ACTION,
    CONSTRAINT post_like_has_one_post
        FOREIGN KEY (post_id)
            REFERENCES posts(id)
                ON DELETE CASCADE
                ON UPDATE NO ACTION
);

SELECT diesel_manage_updated_at('post_likes');