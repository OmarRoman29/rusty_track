use chrono::NaiveDate;
use diesel::prelude::*;

#[allow(non_snake_case, dead_code)]

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::consulta)]
pub struct Consulta{
    idConsulta: u32,
    peso: f32,
    altura: f32,
    presionS: f32,
    presionD: f32,
    temperatura: f32,
    sintomas: String,
    idPaciente: u32,
    idDoctor: u32,
}
