extern crate diesel;
use crate::error::BlockingError;
use actix_web::error;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use serde::Serialize;
/* use futures::future::{ok, Ready};
use futures::Future; */
use std::env;
use tera::Tera;

pub mod models;
pub mod schema;
//use crate::schema;
//use self::models::{Jugador, NewJugador, NewJugadorHandler, NewPost, NewPostHandler, Post};
use self::models::{NewPost, NewPostHandler, Post};
use self::schema::{jugadores, ligas, posts};
use self::schema::{jugadores::dsl::*, ligas::dsl::*, posts::dsl::*};
use models::jugadores::{Jugador, NewJugador, NewJugadorHandler};
use models::ligas::{Liga, NewLiga, NewLigaHandler};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
// Enum para definir tipos de errores relacionados con la API

// Enum para definir errores relacionados con la carga JSON de la solicitud
enum JsonPayloadError {
    Deserialize,
    // Agrega más variantes de error según tus necesidades
}
#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");

    // Consulta para obtener todos los registros
    match web::block(move || posts.load::<Post>(&mut conn)).await {
        Ok(data) => {
            let data = data.unwrap();

            // Enviamos, a través del contexto, los datos al HTML
            let mut ctx = tera::Context::new();
            ctx.insert("posts", &data);

            // Pasamos los datos al template index.html
            HttpResponse::Ok()
                .content_type("text/html")
                .body(template_manager.render("index.html", &ctx).unwrap())
        }
        Err(err) => HttpResponse::Ok().body("Error al recibir la data"),
    }
}

// Capturamos el parámetro por URL
#[get("/blog/{blog_slug}")]
async fn get_post(
    pool: web::Data<DbPool>,
    template_manager: web::Data<tera::Tera>,
    blog_slug: web::Path<String>,
) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");

    let url_slug = blog_slug.into_inner();
    println!("Valor de url_slug: {}", url_slug);
    match web::block(move || posts.filter(slug.eq(url_slug)).load::<Post>(&mut conn)).await {
        Ok(data) => {
            let data = data.unwrap();
            println!("{:?}", data);
            // Si el post no existe, devolvemos 404
            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let data = &data[0];

            // Enviamos, a través del contexto, los datos del post al HTML
            let mut ctx = tera::Context::new();
            ctx.insert("post", data);

            HttpResponse::Ok()
                .content_type("text/html")
                .body(template_manager.render("posts.html", &ctx).unwrap())
        }
        Err(err) => HttpResponse::Ok().body("Error al recibir la data"),
    }
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    // Traemos el POOL para disponer de la conexión a la BBDD
    let mut conn = pool.get().expect("Problemas al traer la base de datos");

    // Utiliamos la función creada en el modelo para crear un nuevo registro y devolverlo
    match web::block(move || Post::create_post(&mut conn, &item)).await {
        Ok(data) => {
            return HttpResponse::Ok().body(format!("{:?}", data));
        }
        Err(err) => HttpResponse::Ok().body("Error al recibir la data"),
    }
}

#[derive(Serialize)]
struct MyObj {
    message: String,
}

#[post("/new_jugador")]
async fn new_jugador(
    pool: web::Data<DbPool>,
    item: web::Json<NewJugadorHandler>,
) -> impl Responder {
    // Traemos el POOL para disponer de la conexión a la BBDD
    let mut conn = pool.get().expect("Problemas al traer la base de datos");

    // Utiliamos la función creada en el modelo para crear un nuevo registro y devolverlo
    match web::block(move || Jugador::create_jugador(&mut conn, &item)).await {
        Ok(data) => {
            return HttpResponse::Ok().body(format!("{:?}", data));
        }
        Err(err) => HttpResponse::Ok().body("Error al recibir la data"),
    }
}

#[derive(Serialize)]
struct MyObj2 {
    message: String,
}

#[post("/new_liga")]
async fn new_liga(pool: web::Data<DbPool>, item: web::Json<NewLigaHandler>) -> impl Responder {
    // Traemos el POOL para disponer de la conexión a la BBDD
    let mut conn = pool.get().expect("Problemas al traer la base de datos");
    let response = MyObj2 {
        message: String::from("Nueva Liga creada exitosamente"),
    };
    // Utiliamos la función creada en el modelo para crear un nuevo registro y devolverlo
    match web::block(move || Liga::create_liga(&mut conn, &item)).await {
        Ok(data) => {
            //return HttpResponse::Ok().body(format!("{:?}", data));
            return HttpResponse::Ok().json(response);
        }
        Err(err) => HttpResponse::Ok().body("Error al recibir la data"),
    }
}

#[get("/tera_test")]
async fn tera_test(template_manager: web::Data<tera::Tera>) -> impl Responder {
    // Creamos un contexto para pasarle datos al template
    let mut ctx = tera::Context::new();

    // Enviamos el template que queremos localizándolo por su nombre
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template_manager.render("tera_test.html", &ctx).unwrap())
}

// Nueva ruta para la página crear_post
#[get("/crear_post")]
async fn crear_post(template_manager: web::Data<tera::Tera>) -> impl Responder {
    // Renderizamos la página crear_post.html
    HttpResponse::Ok().content_type("text/html").body(
        template_manager
            .render("create_post.html", &tera::Context::new())
            .unwrap(),
    )
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DB url variable doesn't found");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .build(manager)
        .expect("No se pudo construir la pool");

    HttpServer::new(move || {
        // Instanciamos TERA y le indicamos en qué directorio buscar los templates
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            //.wrap_fn(middleware)
            .service(index)
            .service(new_post)
            .service(tera_test)
            .service(get_post)
            .service(crear_post)
            .service(new_jugador)
            .service(new_liga)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
    })
    .bind(("localhost", 9900))?
    .run()
    .await
}
