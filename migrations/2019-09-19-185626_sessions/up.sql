CREATE TABLE sessions (
    session_id serial PRIMARY KEY,
    identifier text NOT NULL,
    user_id int NOT NULL REFERENCES users,
    start_time timestamp with time zone NOT NULL DEFAULT now()
);
