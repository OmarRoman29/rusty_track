CREATE TABLE consulta(
    idConsulta INT(4) ZEROFILL PRIMARY KEY AUTO_INCREMENT,
    fecha DATE,
    peso FLOAT,
    altura FLOAT,
    presionS FLOAT,
    presionD FLOAT,
    temperatura FLOAT,
    sintomas TEXT,
    idPaciente INT(4) ZEROFILL NOT NULL,
    idDoctor INT(4) ZEROFILL NOT NULL,
    FOREIGN KEY(idPaciente) REFERENCES paciente(idPaciente),
    FOREIGN KEY(idDoctor) REFERENCES doctor(idDoctor)
);
