CREATE TYPE device_item AS ENUM (
    'socket', 'thermometer'
);

CREATE TABLE devices (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  parent_room VARCHAR NOT NULL,
  type device_item NOT NULL,
  data TEXT NOT NULL
);
