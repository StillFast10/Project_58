from .models import Usuario

class DataService:
    def __init__(self):
        self.usuarios = []

    def agregar_usuario(self, usuario: Usuario):
        if any(u.id == usuario.id or u.dni == usuario.dni for u in self.usuarios):
            raise ValueError("Usuario con ese ID o DNI ya existe")
        self.usuarios.append(usuario)

    def obtener_usuario_por_id(self, id: int):
        return next((u for u in self.usuarios if u.id == id), None)

    def obtener_usuario_por_email(self, email: str):
        return next((u for u in self.usuarios if u.email == email), None)

    def listar_usuarios(self):
        return self.usuarios

    def eliminar_usuario(self, id: int):
        self.usuarios = [u for u in self.usuarios if u.id != id]
