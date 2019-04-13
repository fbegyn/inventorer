table! {
    items (id) {
        id -> Integer,
        name -> Text,
        location -> Text,
        team -> Text,
        amount -> Nullable<Integer>,
        barcode -> Nullable<Integer>,
        for_rent -> Bool,
    }
}
