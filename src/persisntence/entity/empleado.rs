use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;

#[allow(non_snake_case)]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::Empleado)]
pub struct Empleado{
    pub idEmp: u32,
    pub nombre: String,
    pub apellidoP: String,
    pub apellidoM: String,
    pub fNacimiento: NaiveDate,
    pub fIngreso: NaiveDate,
    pub categoria: String,
    pub sueldo: BigDecimal,
}
