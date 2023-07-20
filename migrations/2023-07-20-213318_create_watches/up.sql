CREATE TABLE watches
(
    id              VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL,
    brand           VARCHAR(255) NOT NULL,
    model           VARCHAR(255) NOT NULL,
    diameter        VARCHAR(255) NOT NULL,
    lug_width       VARCHAR(255) NOT NULL,
    case_material   VARCHAR(255) NOT NULL,
    mechanism_model VARCHAR(255) NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_watches_id PRIMARY KEY (id)
);

SELECT diesel_manage_updated_at('watches');

CREATE INDEX IF NOT EXISTS watch_brand           ON watches USING BTREE(brand);
CREATE INDEX IF NOT EXISTS watch_model           ON watches USING BTREE(model);
CREATE INDEX IF NOT EXISTS watch_diameter        ON watches USING BTREE(diameter);
CREATE INDEX IF NOT EXISTS watch_lug_width       ON watches USING BTREE(lug_width);
CREATE INDEX IF NOT EXISTS watch_case_material   ON watches USING BTREE(case_material);
CREATE INDEX IF NOT EXISTS watch_mechanism_model ON watches USING BTREE(mechanism_model);