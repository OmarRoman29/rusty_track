use diesel::prelude::*;

#[allow(non_snake_case)]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::Medicamento)]
pub struct Medicamento{
    pub idMe: u32,
    pub nombre: String,
    pub nombreC: String,
}
