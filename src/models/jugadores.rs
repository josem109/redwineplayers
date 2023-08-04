use crate::schema::jugadores;
use chrono::NaiveDate;
use std::error::Error;
use validator::Validate;
/* use diesel::dsl::sql;
use diesel::expression::AsExpression;
use diesel::pg::Pg; */
use diesel::sql_types::{Date, Nullable};
use diesel::Insertable;
use diesel::PgConnection;
use diesel::Queryable;
use serde::{Deserialize, Serialize}; // Usa `crate` para referirte al mismo nivel del proyecto
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

#[derive(Debug)]
pub enum MyError {
    FechaFutura,
    FormatoInvalido,
    // ... otras variantes de error si es necesario
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::FechaFutura => write!(f, "La fecha no puede ser en el futuro"),
            MyError::FormatoInvalido => write!(f, "El formato de fecha es inválido"),
            // ... otras implementaciones Display si es necesario
        }
    }
}
impl std::error::Error for MyError {}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewJugadorHandler {
    pub nombre: String,
    pub fecha_nacimiento: Option<NaiveDate>,
}

// Importamos el esquema de la BBDD
//use super::schema::jugadores;
use diesel::prelude::*;

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
        // Validar la fecha de nacimiento
        if let Some(fecha_nacimiento) = jugador.fecha_nacimiento {
            let fecha_actual = chrono::Utc::now().naive_utc().date();

            if fecha_nacimiento > fecha_actual {
                // La fecha está en el futuro
                return Err(diesel::result::Error::QueryBuilderError(Box::new(
                    MyError::FechaFutura,
                )));
            }

            // Validar el formato de la fecha
            if fecha_nacimiento.format("%Y-%m-%d").to_string() != fecha_nacimiento.to_string() {
                // El formato de la fecha no es válido
                return Err(diesel::result::Error::QueryBuilderError(Box::new(
                    MyError::FormatoInvalido,
                )));
            }
        }
        let new_jugador = NewJugador {
            nombre: &jugador.nombre,
            fecha_nacimiento: &jugador.fecha_nacimiento.unwrap_or_default(),
        };

        diesel::insert_into(jugadores::table)
            .values(new_jugador)
            .get_result::<Jugador>(conn)
    }
}
