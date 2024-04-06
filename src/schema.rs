// @generated automatically by Diesel CLI.

diesel::table! {
    #[allow(non_snake_case)]
    Cita (idCita) {
        idCita -> Unsigned<Integer>,
        fecha -> Date,
        #[max_length = 45]
        sintomas -> Varchar,
        presionD -> Nullable<Integer>,
        presionS -> Nullable<Integer>,
        temperatura -> Nullable<Float>,
        peso -> Nullable<Float>,
        idEmp -> Unsigned<Integer>,
        idPac -> Unsigned<Integer>,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Empleado (idEmp) {
        idEmp -> Unsigned<Integer>,
        #[max_length = 20]
        nombre -> Varchar,
        #[max_length = 20]
        apellidoP -> Varchar,
        #[max_length = 20]
        apellidoM -> Varchar,
        fNacimiento -> Date,
        fIngreso -> Date,
        #[max_length = 10]
        categoria -> Varchar,
        sueldo -> Float,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Medicamento (idMe) {
        idMe -> Unsigned<Integer>,
        #[max_length = 25]
        nombre -> Varchar,
        #[max_length = 45]
        nombreC -> Varchar,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Paciente (idPac) {
        idPac -> Unsigned<Integer>,
        #[max_length = 15]
        nombre -> Varchar,
        #[max_length = 15]
        apellidoP -> Varchar,
        #[max_length = 15]
        apellidoM -> Varchar,
        fNacimiento -> Date,
        #[max_length = 25]
        correo -> Nullable<Varchar>,
        #[max_length = 25]
        telefono -> Nullable<Varchar>,
        #[max_length = 50]
        enfermCro -> Nullable<Varchar>,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Presentacion (idPres) {
        idPres -> Unsigned<Integer>,
        #[max_length = 25]
        nombre -> Varchar,
        #[max_length = 10]
        unidades -> Varchar,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Producto (idPro) {
        idPro -> Unsigned<Integer>,
        #[max_length = 25]
        nombre -> Varchar,
        #[max_length = 25]
        laboratorio -> Varchar,
        cantidad -> Float,
        unidadesC -> Nullable<Integer>,
        disponibilidad -> Integer,
        idPres -> Unsigned<Integer>,
        idMe -> Unsigned<Integer>,
    }
}

diesel::joinable!(Cita -> Empleado (idEmp));
diesel::joinable!(Cita -> Paciente (idPac));
diesel::joinable!(Producto -> Medicamento (idMe));
diesel::joinable!(Producto -> Presentacion (idPres));

diesel::allow_tables_to_appear_in_same_query!(
    Cita,
    Empleado,
    Medicamento,
    Paciente,
    Presentacion,
    Producto,
);
