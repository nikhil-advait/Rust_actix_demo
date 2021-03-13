-- Your SQL goes here
CREATE TABLE order_items
(
    item_id         uuid                        NOT NULL PRIMARY KEY,
    order_id        uuid                        NOT NULL REFERENCES orders(order_id),
    description     varchar(255)                NOT NULL,
    qty             integer                     NOT NULL CHECK (qty > 0),
    price           integer                     NOT NULL CHECK (qty > 0),
    created_at      timestamp with time zone    NOT NULL
);

CREATE INDEX order_id_index ON order_items (order_id);