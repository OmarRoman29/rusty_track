CREATE TABLE doctor(
    idDoctor INT(4) ZEROFILL PRIMARY KEY AUTO_INCREMENT,
    nombre VARCHAR(45) NOT NULL,
    apellidoP VARCHAR(45) NOT NULL,
    apellidoM VARCHAR(45) NOT NULL,
    fechaIngreso DATE NOT NULL,
    fechaN DATE NOT NULL,
    sexo VARCHAR(10),
    sueldo DECIMAL NOT NULL
);
