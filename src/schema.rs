// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
       // published -> Bool,
        slug -> Text,
    }
}

diesel::table! {
    jugadores (id) {
        id -> Int4,
        #[max_length = 100]
        nombre -> Varchar,
        fecha_nacimiento -> Nullable<Date>,
        id_posicion -> Nullable<Int4>,
        id_club -> Nullable<Int4>,
        #[max_length = 255]
        url -> Nullable<Varchar>,
        #[max_length = 255]
        foto -> Nullable<Varchar>,
    }
}
