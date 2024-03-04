CREATE TABLE lote(
    idLote INT(4) ZEROFILL PRIMARY KEY AUTO_INCREMENT,
    marca VARCHAR(50),
    laboratio VARCHAR(30),
    presentacion VARCHAR(30) NOT NULL,
    gramaje FLOAT NOT NULL,
    unidades VARCHAR(15), /* mililitros, gramos, etc */
    cantidadCaja INT(4) NOT NULL, /* Cantidad de pastillas, ampollas, etc en el empaque */
    costo DECIMAL NOT NULL,
    caducidad DATE NOT NULL,
    cajasPorLote INT(4) ZEROFILL NOT NULL,
    idMedicamento INT(4) ZEROFILL,
    FOREIGN KEY (idMedicamento) REFERENCES farmaco(idMedicamento)
);
