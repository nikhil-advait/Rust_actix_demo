-- Your SQL goes here
CREATE TABLE orders
(
    order_id    uuid    NOT NULL PRIMARY KEY,
    user_id     uuid    NOT NULL REFERENCES users(user_id),
    note        varchar(500),
    created_at timestamp with time zone NOT NULL
);

CREATE INDEX user_id_index ON orders (user_id);