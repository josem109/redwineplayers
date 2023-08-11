// @generated automatically by Diesel CLI.

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

diesel::table! {
    ligas (id) {
        id -> Int4,
        #[max_length = 100]
        nombre -> Varchar,
        pais_id -> Nullable<Int4>,
        #[max_length = 50]
        formato_competencia -> Nullable<Varchar>,
        #[max_length = 255]
        escudo -> Nullable<Varchar>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        slug -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(jugadores, ligas, posts,);
