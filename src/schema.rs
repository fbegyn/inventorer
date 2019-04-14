table! {
    items (id) {
        id -> Integer,
        name -> Text,
        location_id -> Integer,
        team_id -> Nullable<Integer>,
        amount -> Nullable<Integer>,
        barcode -> Nullable<Text>,
        for_rent -> Bool,
    }
}

table! {
    locations (id) {
        id -> Integer,
        name -> Text,
        warehouse_id -> Nullable<Integer>,
    }
}

table! {
    teams (id) {
        id -> Integer,
        name -> Text,
        teamlead_id -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        team_id -> Nullable<Integer>,
    }
}

table! {
    warehouses (id) {
        id -> Integer,
        name -> Text,
        address -> Nullable<Text>,
        capacity -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    items,
    locations,
    teams,
    users,
    warehouses,
);
