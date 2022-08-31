table! {
    house_rooms (id) {
        id -> Int4,
        house_id -> Int4,
        room_id -> Int4,
    }
}

table! {
    houses (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    rooms (id) {
        id -> Int4,
        name -> Varchar,
        device_names -> Array<Text>,
    }
}

joinable!(house_rooms -> houses (house_id));
joinable!(house_rooms -> rooms (room_id));

allow_tables_to_appear_in_same_query!(
    house_rooms,
    houses,
    rooms,
);
