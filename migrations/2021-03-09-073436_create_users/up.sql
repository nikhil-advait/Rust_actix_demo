CREATE TABLE users (
  user_id     uuid                      NOT NULL  PRIMARY KEY,
  first_name  varchar(255)              NOT NULL,
  last_name   varchar(255)              NOT NULL,
  email       varchar(255)              NOT NULL,
  password    varchar(255)              NOT NULL,
  created_at  timestamp with time zone  NOT NULL
)