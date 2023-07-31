-- Add migration script here
CREATE TABLE task (
    id INTEGER PRIMARY KEY,
    task VARCHAR(255)[] NOT NULL,
    stat INTEGER DEFAULT 0 NOT NULL,
    CHECK(stat > -1),
    CHECK(stat < 3)
);