from enum import Enum

class Genero(Enum):
    MASCULINO = "Masculino"
    FEMENINO = "Femenino"
    JOSELITO = "Joselito"

class Usuario:
    def __init__(self, id: int, nombre: str, edad: int, genero: Genero,
                 grado: int, dni: int, password: str, email: str):
        self.id = id
        self.nombre = nombre
        self.edad = edad
        self.genero = genero
        self.grado = grado
        self.dni = dni
        self.password = password
        self.email = email

    def __repr__(self):
        return f"<Usuario {self.id}: {self.nombre}, {self.email}>"
