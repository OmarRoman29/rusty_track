CREATE TABLE farmaco(
    idMedicamento INT(4) ZEROFILL PRIMARY KEY AUTO_INCREMENT,
    nombreComercial VARCHAR(50) NOT NULL,
    subtancia VARCHAR(50) NOT NULL,
    unidadesInventario INT(4) ZEROFILL
);
