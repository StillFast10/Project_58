from flask import Flask, request, jsonify
from dataservice.models import Usuario, Genero
from dataservice.service import DataService

app = Flask(__name__)
ds = DataService()

def usuario_to_dict(u):
    return {
        "id": u.id,
        "nombre": u.nombre,
        "edad": u.edad,
        "genero": u.genero.value if hasattr(u.genero, "value") else str(u.genero),
        "grado": u.grado,
        "dni": u.dni,
        "password": u.password,
        "email": u.email,
    }

@app.route("/usuarios", methods=["POST"])
def crear_usuario():
    data = request.get_json() or {}
    required = ["id","nombre","edad","genero","grado","dni","password","email"]
    missing = [k for k in required if k not in data]
    if missing:
        return jsonify({"error":"Faltan campos","missing": missing}), 400

    # normalizar/convertir genero (acepta por name o por value, case-insensitive)
    genero_input = data["genero"]
    genero = None
    try:
        genero = Genero[genero_input.upper()]  # por nombre (MASCULINO, JOSELITO, ...)
    except Exception:
        genero = next((g for g in Genero if g.value.lower() == genero_input.lower()), None)
    if genero is None:
        return jsonify({"error":"Genero inválido","allowed":[g.value for g in Genero]}), 400

    try:
        usuario = Usuario(
            id=int(data["id"]),
            nombre=data["nombre"],
            edad=int(data["edad"]),
            genero=genero,
            grado=int(data["grado"]),
            dni=int(data["dni"]),
            password=data["password"],
            email=data["email"]
        )
        ds.agregar_usuario(usuario)
    except ValueError as e:
        # aquí controlamos el "Usuario ya existe"
        return jsonify({"error": str(e)}), 409
    except Exception as e:
        return jsonify({"error":"Error interno","detail": str(e)}), 500

    return jsonify({"mensaje":"Usuario creado","usuario": usuario_to_dict(usuario)}), 201

@app.route("/usuarios/<int:user_id>", methods=["GET"])
def obtener_usuario(user_id):
    usuario = ds.obtener_usuario_por_id(user_id)
    if usuario:
        return jsonify(usuario_to_dict(usuario)), 200
    return jsonify({"error":"Usuario no encontrado"}), 404

@app.route("/usuarios", methods=["GET"])
def listar_usuarios():
    return jsonify([usuario_to_dict(u) for u in ds.listar_usuarios()]), 200

@app.route("/usuarios/<int:user_id>", methods=["DELETE"])
def eliminar_usuario(user_id):
    if ds.obtener_usuario_por_id(user_id):
        ds.eliminar_usuario(user_id)
        return jsonify({"mensaje":"Usuario eliminado"}), 200
    return jsonify({"error":"Usuario no encontrado"}), 404

# endpoint de ayuda solo para pruebas (borra todo)
@app.route("/debug/clear", methods=["POST"])
def clear_all():
    ds.usuarios = []
    return jsonify({"mensaje":"Todos los usuarios eliminados"}), 200

if __name__ == "__main__":
    app.run(host="127.0.0.1", port=9000, debug=True)
