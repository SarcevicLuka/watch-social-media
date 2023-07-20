CREATE TABLE watch_images
(
    id              VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    watch_id        VARCHAR(36) NOT NULL,
    title           VARCHAR(36) NOT NULL,
    "data"          TEXT DEFAULT NULL,
    CONSTRAINT pk_watch_images_id PRIMARY KEY (id),
    CONSTRAINT one_watch_has_many_images
        FOREIGN KEY (watch_id)
            REFERENCES watches(id)
                ON DELETE CASCADE
                ON UPDATE NO ACTION
);

SELECT diesel_manage_updated_at('watch_images');

CREATE INDEX IF NOT EXISTS watch_images_title ON watch_images USING BTREE(title);
CREATE INDEX IF NOT EXISTS watch_images_data  ON watch_images USING BTREE("data");