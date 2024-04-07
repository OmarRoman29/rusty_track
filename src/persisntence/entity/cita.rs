use chrono::NaiveDate;
use diesel::prelude::*;

#[allow(non_snake_case)]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::Cita)]
pub struct Cita{
    pub idCita: u32,
    pub fecha: NaiveDate,
    pub sintomas: String,
    pub presionD: Option<i32>,
    pub presionS: Option<i32>,
    pub temperatura: Option<f32>,
    pub peso: Option<f32>,
    pub idEmp: u32,
    pub idPac: u32,
}
