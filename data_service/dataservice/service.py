import psycopg2
from psycopg2.extras import RealDictCursor
from dataservice.models import Usuario

class DataService:
    def __init__(self):
        # Ajusta estos valores según tu instalación de PostgreSQL
        self.conn = psycopg2.connect(
            host="localhost",
            database="counterclock",
            user="postgres",       # tu usuario de PostgreSQL
            password="Cartoslol2.4" # recuerda que esto lo tienes que hacer con tu contra del PostgreSQL
        )

    def agregar_usuario(self, usuario: Usuario):
        with self.conn.cursor() as cur:
            # Verificar si ya existe el usuario
            cur.execute("SELECT id FROM usuarios WHERE id=%s OR dni=%s;", (usuario.id, usuario.dni))
            if cur.fetchone():
                raise ValueError("Usuario con ese ID o DNI ya existe")

            cur.execute("""
                INSERT INTO usuarios (id, nombre, edad, genero, grado, dni, password, email)
                VALUES (%s, %s, %s, %s, %s, %s, %s, %s);
            """, (
                usuario.id,
                usuario.nombre,
                usuario.edad,
                usuario.genero.value,
                usuario.grado,
                usuario.dni,
                usuario.password,
                usuario.email
            ))
            self.conn.commit()

    def obtener_usuario_por_id(self, id: int):
        with self.conn.cursor(cursor_factory=RealDictCursor) as cur:
            cur.execute("SELECT * FROM usuarios WHERE id=%s;", (id,))
            row = cur.fetchone()
            return row if row else None

    def listar_usuarios(self):
        with self.conn.cursor(cursor_factory=RealDictCursor) as cur:
            cur.execute("SELECT * FROM usuarios;")
            return cur.fetchall()

    def eliminar_usuario(self, id: int):
        with self.conn.cursor() as cur:
            cur.execute("DELETE FROM usuarios WHERE id=%s;", (id,))
            self.conn.commit()
