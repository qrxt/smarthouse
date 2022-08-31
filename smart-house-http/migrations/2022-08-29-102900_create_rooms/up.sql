CREATE TABLE rooms (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  device_names TEXT[] NOT NULL
);
