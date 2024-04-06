-- Your SQL goes here

DROP TABLE IF EXISTS Empleado;
CREATE TABLE Empleado(
    idEmp INT(4) ZEROFILL PRIMARY KEY AUTO_INCREMENT,
    nombre VARCHAR(20) NOT NULL,
    apellidoP VARCHAR(20) NOT NULL,
    apellidoM VARCHAR(20) NOT NULL,
    fNacimiento DATE NOT NULL,
    fIngreso DATE NOT NULL,
    categoria VARCHAR(10) NOT NULL,
    sueldo FLOAT NOT NULL
);

DROP TABLE IF EXISTS Medicamento;
CREATE TABLE Medicamento(
    idMe INT(4) ZEROFILL PRIMARY KEY AUTO_INCREMENT,
    nombre VARCHAR(25) NOT NULL,
    nombreC VARCHAR(45) NOT NULL
);

DROP TABLE IF EXISTS Presentacion;
CREATE TABLE Presentacion(
    idPres INT(4) ZEROFILL PRIMARY KEY AUTO_INCREMENT,
    nombre VARCHAR(25) NOT NULL,
    unidades VARCHAR(10) NOT NULL /* gramos, mililitros, etc*/
);


DROP TABLE IF EXISTS Producto;
CREATE TABLE Producto(
    idPro INT(4) ZEROFILL PRIMARY KEY AUTO_INCREMENT,
    nombre VARCHAR(25) NOT NULL,
    laboratorio VARCHAR(25) NOT NULL,
    cantidad FLOAT NOT NULL, /* dato designado para indicar gramaje o parecidos */
    unidadesC INT(4), /* Unidades por caja tipo, 5 tabletas */
    disponibilidad INT(4) NOT NULL, /* unidades en inventario */
    idPres INT(4) ZEROFILL NOT NULL,
    idMe INT(4) ZEROFILl NOT NULL,
    FOREIGN KEY(idPres) REFERENCES Presentacion(idPres),
    FOREIGN KEY(idMe) REFERENCES Medicamento(idME)
);

DROP TABLE IF EXISTS Paciente;
CREATE TABLE Paciente(
    idPac INT(4) ZEROFILL PRIMARY KEY AUTO_INCREMENT,
    nombre VARCHAR(15) NOT NULL,
    apellidoP VARCHAR(15) NOT NULL,
    apellidoM VARCHAR(15) NOT NULL,
    fNacimiento DATE NOT NULL,
    correo VARCHAR(25),
    telefono VARCHAR(25),
    enfermCro VARCHAR(50)
);

DROP TABLE IF EXISTS Cita;
CREATE TABLE Cita(
    idCita INT(4) ZEROFILL PRIMARY KEY AUTO_INCREMENT,
    fecha DATE NOT NULL,
    sintomas VARCHAR(45) NOT NULL,
    presionD INT,
    presionS INT,
    temperatura FLOAT,
    peso FLOAT,
    idEmp INT(4) ZEROFILL NOT NULL,
    idPac INT(4) ZEROFILL NOT NULL,
    FOREIGN KEY(idEmp) REFERENCES Empleado(idEmp),
    FOREIGN KEY(idPac) REFERENCES Paciente(idPac)
);
