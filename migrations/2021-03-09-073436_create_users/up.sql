CREATE TABLE users (
  user_id uuid NOT NULL PRIMARY KEY,
  first_name  character varying(255)    NOT NULL,
  last_name   character varying(255)    NOT NULL,
  email       character varying(255)    NOT NULL,
  password    character varying(255)    NOT NULL,
  created_at  timestamp with time zone  NOT NULL
)