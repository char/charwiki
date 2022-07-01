CREATE TABLE sessions (
  token TEXT PRIMARY KEY NOT NULL,
  user_uuid TEXT NOT NULL,
  expires INTEGER NOT NULL,
  FOREIGN KEY (user_uuid) REFERENCES user(uuid)
) STRICT;
