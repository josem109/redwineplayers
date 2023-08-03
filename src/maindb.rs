// Indicamos que vamos a utilizar macros
#[macro_use]
// Importamos Diesel
extern crate diesel;

// Importamos la conexión con PostgreSQL
use diesel::pg::PgConnection;
use diesel::prelude::*;

// Importamos librerias para leer variables de entorno
use dotenvy::dotenv;
use std::env;

// Importamos esquemas de BBDD y modelos
pub mod models;
pub mod schema;

fn main() {
    // Indicamos que vamos a utilizar el esquema de Posts y el modelo
    use self::models::{NewPost, Post, PostSimplificado};
    use self::schema::posts;
    use self::schema::posts::dsl::*;
    println!("Hola");
    // Lectura de variables de entorno
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("La variable de entorno DATABASE_URL no existe.");

    // Conexión con la BBDD
    let mut conn = PgConnection::establish(&db_url)
        .expect("No se ha podido establecer la conexión con la base de datos.");

    // Realizamos la consulta equivalente a: SELECT * FROM posts;
    /*     let posts_result = posts
    .load::<Post>(&mut conn)
    .expect("Error en la consulta SQL."); */

    // Insertamos el primer registro en la BBDD
    /*     let new_post = NewPost {
        title: "Mi Cuarto post",
        body: "Lorem ipsum...",
        //slug: "primer-post",
    };
    diesel::insert_into(posts::table)
        .values(new_post)
        .get_result::<Post>(&mut conn)
        .expect("Falló el insert en la BBDD"); */
    //Borrar un registro por ID
    /*     diesel::delete(posts.filter(id.eq(2)))
    .execute(&mut conn)
    .expect("Ha fallado la eliminacion del registro"); */
    //Borrar un registro por Like
    /*     diesel::delete(posts.filter(title.like("%post%")))
        .execute(&mut conn)
        .expect("Ha fallado la eliminacion del registro");
    // Realizamos la consulta equivalente a: SELECT * FROM posts; */
    println!("Query sin limites");
    let posts_result = posts
        .load::<Post>(&mut conn)
        .expect("Error en la consulta SQL.");
    // Iteramos los resultados de la consulta
    for post in posts_result {
        println!("{:?}", post);
    }
    //Devuelve un solo registro
    println!("Query con limites");
    let posts_result = posts
        .order(id.desc())
        .limit(1)
        .load::<Post>(&mut conn)
        .expect("Error en la consulta SQL.");

    // Iteramos los resultados de la consulta
    for post in posts_result {
        println!("{:?}", post);
    }

    //Select Title, Body from posts limit 1
    println!("Query con columnas especificas");
    let posts_result = posts
        .select((title, body))
        .limit(1)
        .load::<PostSimplificado>(&mut conn)
        .expect("Error en la consulta SQL.");

    // Iteramos los resultados de la consulta
    for post in posts_result {
        println!("{:?}", post);
    }

    //Select valor especifico where id=2
    println!("Query con where");
    let posts_result = posts
        .filter(id.eq(2))
        .limit(1)
        .load::<Post>(&mut conn)
        .expect("Error en la consulta SQL.");

    // Iteramos los resultados de la consulta
    for post in posts_result {
        println!("{:?}", post);
    }
}
