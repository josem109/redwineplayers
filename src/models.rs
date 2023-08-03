use chrono::NaiveDate;
use diesel::dsl::sql;
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::sql_types::{Date, Nullable};
use diesel::Insertable;
use diesel::PgConnection;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
// Importamos los tipos de datos necesarios para definir el modelo
//use diesel::sql_types::*;
// Macro para indicar que los registros de la BBDD tendrán la misma forma que la estructura.
#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    //pub published: bool,
    pub slug: String,
}

// Definimos la estructura para el modelo "Jugador"
#[derive(Debug, Queryable)]
pub struct Jugador {
    pub id: i32,
    pub nombre: String,
    pub fecha_nacimiento: Option<NaiveDate>, // Utilizamos Option para campos nullable
    pub id_posicion: Option<i32>,
    pub id_club: Option<i32>,
    pub url: Option<String>,
    pub foto: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = jugadores)]
pub struct NewJugadorHandler {
    pub nombre: String,
    pub fecha_nacimiento: Option<NaiveDate>,
}

#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct PostSimplificado {
    pub title: String,
    pub body: String,
    pub slug: String,
}

// Importamos el esquema de la BBDD
use super::schema::jugadores;
use super::schema::posts;
use diesel::prelude::*;

// Macro para indicar que la estructura servirá que insert en la BBDD
#[derive(Insertable)]
//#[table_name = "posts"]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}

impl Post {
    pub fn slugify(title: &String) -> String {
        return title.replace(" ", "-").to_lowercase();
    }
    pub fn create_post<'a>(
        conn: &mut PgConnection,
        post: &NewPostHandler,
    ) -> Result<Post, diesel::result::Error> {
        let slug = Post::slugify(&post.title.clone());
        println!("{:?}", slug);
        let new_post = NewPost {
            title: &post.title,
            slug: &slug,
            body: &post.body,
            //published: Some(false),
        };

        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result::<Post>(conn)
    }
}

// Macro para indicar que la estructura servirá que insert en la BBDD
#[derive(Insertable)]
//#[table_name = "posts"]
#[diesel(table_name = jugadores)]
pub struct NewJugador<'a> {
    pub nombre: &'a str,
    pub fecha_nacimiento: &'a NaiveDate,
}

impl Jugador {
    pub fn create_jugador<'a>(
        conn: &mut PgConnection,
        jugador: &NewJugadorHandler,
    ) -> Result<Jugador, diesel::result::Error> {
        let new_jugador = NewJugador {
            nombre: &jugador.nombre,
            fecha_nacimiento: &jugador.fecha_nacimiento.unwrap_or_default(),
        };

        diesel::insert_into(jugadores::table)
            .values(new_jugador)
            .get_result::<Jugador>(conn)
    }
}
