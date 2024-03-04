CREATE TABLE paciente(
    idPaciente INT(4) ZEROFILL PRIMARY KEY AUTO_INCREMENT,
    nombre VARCHAR(45) NOT NULL,
    apellidoP VARCHAR(45) NOT NULL,
    apellidoM VARCHAR(45) NOT NULL,
    sexo VARCHAR(10) NOT NULL,
    fechaNac DATE NOT NULL,
    curp VARCHAR(15) NOT NULL,
    tipoS VARCHAR(45) NOT NULL,
    alergias TEXT,
    enfermedadesCronicas TEXT
);

