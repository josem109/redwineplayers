use crate::schema::ligas;
use chrono::NaiveDate;
use diesel::sql_types::{Date, Nullable};
use diesel::Insertable;
use diesel::PgConnection;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use std::error::Error;
use validator::Validate; // Usa `crate` para referirte al mismo nivel del proyecto
                         // Definimos la estructura para el modelo "Jugador"

#[derive(Debug, Queryable)]
pub struct Liga {
    pub id: i32,
    pub nombre: String,
    pub pais_id: Option<i32>,
    pub formato_competencia: Option<String>,
    pub escudo: Option<String>,
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
pub struct NewLigaHandler {
    pub nombre: String,
    pub id_pais: Option<i32>,
    pub formato_competencia: String,
    pub escudo: String,
}

// Importamos el esquema de la BBDD
//use super::schema::jugadores;
use diesel::prelude::*;

// Macro para indicar que la estructura servirá que insert en la BBDD
#[derive(Insertable)]
//#[table_name = "posts"]
#[diesel(table_name = ligas)]
pub struct NewLiga<'a> {
    pub nombre: &'a str,
    pub formato_competencia: &'a str,
    pub escudo: &'a str,
}

impl Liga {
    pub fn create_liga<'a>(
        conn: &mut PgConnection,
        liga: &NewLigaHandler,
    ) -> Result<Liga, diesel::result::Error> {
        // Validar la fecha de nacimiento
        let new_liga = NewLiga {
            nombre: &liga.nombre,
            formato_competencia: &liga.formato_competencia,
            escudo: &liga.escudo,
        };

        diesel::insert_into(ligas::table)
            .values(new_liga)
            .get_result::<Liga>(conn)
    }
}
