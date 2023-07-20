CREATE TABLE comments
(
    id         VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    user_id    VARCHAR(36) NOT NULL,
    post_id    VARCHAR(36) NOT NULL,
    "text"     TEXT NOT NULL,
    score      INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_comments_id PRIMARY KEY (id),
    CONSTRAINT comment_like_has_one_user
        FOREIGN KEY (user_id)
            REFERENCES users(id)
                ON DELETE CASCADE
                ON UPDATE NO ACTION,
    CONSTRAINT comment_has_one_post
        FOREIGN KEY (post_id)
            REFERENCES posts(id)
                ON DELETE CASCADE
                ON UPDATE NO ACTION
);

SELECT diesel_manage_updated_at('comments');

CREATE INDEX IF NOT EXISTS comment_text ON comments USING BTREE("text");
CREATE INDEX IF NOT EXISTS comment_score   ON comments USING BTREE(score);