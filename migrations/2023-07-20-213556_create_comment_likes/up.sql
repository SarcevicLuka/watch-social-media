CREATE TABLE comment_likes
(
    id         VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    user_id    VARCHAR(36) NOT NULL,
    comment_id VARCHAR(36) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_comment_likes_id PRIMARY KEY (id),
    CONSTRAINT comment_like_has_one_user
        FOREIGN KEY (user_id)
            REFERENCES users(id)
                ON DELETE CASCADE
                ON UPDATE NO ACTION,
    CONSTRAINT comment_like_has_one_comment
        FOREIGN KEY (comment_id)
            REFERENCES comments(id)
                ON DELETE CASCADE
                ON UPDATE NO ACTION
);

SELECT diesel_manage_updated_at('comment_likes');