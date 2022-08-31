CREATE TABLE house_rooms (
  id SERIAL PRIMARY KEY,
  house_id INTEGER NOT NULL,
  FOREIGN KEY (house_id) REFERENCES houses(id),
  room_id INTEGER NOT NULL,
  FOREIGN KEY (room_id) REFERENCES rooms(id)
);
