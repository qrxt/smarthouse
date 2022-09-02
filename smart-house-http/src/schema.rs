table! {
    use diesel::sql_types::*;
    use crate::device::Device_item;

    devices (id) {
        id -> Int4,
        name -> Varchar,
        parent_room -> Int4,
        #[sql_name = "type"]
        type_ -> Device_item,
        data -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::device::Device_item;

    house_rooms (id) {
        id -> Int4,
        house_id -> Int4,
        room_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::device::Device_item;

    houses (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::device::Device_item;

    rooms (id) {
        id -> Int4,
        name -> Varchar,
        device_names -> Array<Text>,
    }
}

joinable!(devices -> rooms (parent_room));
joinable!(house_rooms -> houses (house_id));
joinable!(house_rooms -> rooms (room_id));

allow_tables_to_appear_in_same_query!(
    devices,
    house_rooms,
    houses,
    rooms,
);
