use chrono::NaiveDate;
use diesel::prelude::*;

#[allow(non_snake_case)]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::Paciente)]
pub struct Paciente{
    pub idPac: u32,
    pub nombre: String,
    pub apellidoP: String,
    pub apellidoM: String,
    pub fNacimiento: NaiveDate,
    pub correo: Option<String>,
    pub telefono: Option<String>,
    pub enfermCro: Option<String>,
}
