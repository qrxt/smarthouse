CREATE TYPE device_item AS ENUM (
    'socket', 'thermometer'
);

CREATE TABLE devices (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  parent_room INTEGER NOT NULL,
  FOREIGN KEY (parent_room) REFERENCES rooms(id),
  type device_item NOT NULL,
  data TEXT NOT NULL
);
