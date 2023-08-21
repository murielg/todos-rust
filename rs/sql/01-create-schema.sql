-- TODO status
CREATE TYPE todo_status_enum AS ENUM (
    'open',
    'closed'
);

-- TODOs
CREATE TABLE todo (
    id bigserial,
    cid bigint NOT NULL, -- this is the user id
    ctime timestamp with time zone DEFAULT now(),
    title text NOT NULL,
    status todo_status_enum NOT NULL DEFAULT 'open'
);

ALTER SEQUENCE todo_id_seq RESTART WITH 1000;