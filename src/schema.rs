// @generated automatically by Diesel CLI.

diesel::table! {
    consulta (idConsulta) {
        idConsulta -> Unsigned<Integer>,
        fecha -> Nullable<Date>,
        peso -> Nullable<Float>,
        altura -> Nullable<Float>,
        presionS -> Nullable<Float>,
        presionD -> Nullable<Float>,
        temperatura -> Nullable<Float>,
        sintomas -> Nullable<Text>,
        idPaciente -> Unsigned<Integer>,
        idDoctor -> Unsigned<Integer>,
    }
}

diesel::table! {
    doctor (idDoctor) {
        idDoctor -> Unsigned<Integer>,
        #[max_length = 45]
        nombre -> Varchar,
        #[max_length = 45]
        apellidoP -> Varchar,
        #[max_length = 45]
        apellidoM -> Varchar,
        fechaIngreso -> Date,
        fechaN -> Date,
        #[max_length = 10]
        sexo -> Nullable<Varchar>,
        sueldo -> Decimal,
    }
}

diesel::table! {
    farmaco (idMedicamento) {
        idMedicamento -> Unsigned<Integer>,
        #[max_length = 50]
        nombreComercial -> Varchar,
        #[max_length = 50]
        subtancia -> Varchar,
        unidadesInventario -> Nullable<Unsigned<Integer>>,
    }
}

diesel::table! {
    lote (idLote) {
        idLote -> Unsigned<Integer>,
        #[max_length = 50]
        marca -> Nullable<Varchar>,
        #[max_length = 30]
        laboratio -> Nullable<Varchar>,
        #[max_length = 30]
        presentacion -> Varchar,
        gramaje -> Float,
        #[max_length = 15]
        unidades -> Nullable<Varchar>,
        cantidadCaja -> Integer,
        costo -> Decimal,
        caducidad -> Date,
        cajasPorLote -> Unsigned<Integer>,
        idMedicamento -> Nullable<Unsigned<Integer>>,
    }
}

diesel::table! {
    paciente (idPaciente) {
        idPaciente -> Unsigned<Integer>,
        #[max_length = 45]
        nombre -> Varchar,
        #[max_length = 45]
        apellidoP -> Varchar,
        #[max_length = 45]
        apellidoM -> Varchar,
        #[max_length = 10]
        sexo -> Varchar,
        fechaNac -> Date,
        #[max_length = 15]
        curp -> Varchar,
        #[max_length = 45]
        tipoS -> Varchar,
        alergias -> Nullable<Text>,
        enfermedadesCronicas -> Nullable<Text>,
    }
}

diesel::joinable!(consulta -> doctor (idDoctor));
diesel::joinable!(consulta -> paciente (idPaciente));
diesel::joinable!(lote -> farmaco (idMedicamento));

diesel::allow_tables_to_appear_in_same_query!(
    consulta,
    doctor,
    farmaco,
    lote,
    paciente,
);
