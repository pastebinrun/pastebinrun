CREATE TABLE users (
    user_id serial PRIMARY KEY,
    nickname text NOT NULL,
    password text NOT NULL
);
