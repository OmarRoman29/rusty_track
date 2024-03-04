use diesel::prelude::*;
#[allow(non_snake_case, dead_code)]

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::paciente)]
pub struct Paciente{
    idPaciente: u32,
    nombre: String,
    apellidoP: String,
    apellidoM: String,
    sexo: String,
}
