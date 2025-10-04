from flask import Flask, request, jsonify
import psycopg2

app = Flask(__name__)

# Configuración de la base de datos
conn = psycopg2.connect(
    host="localhost",
    database="data_service_db",
    user="postgres",
    password="Cartoslol2.4"
)

cursor = conn.cursor()

@app.route("/", methods=["POST"])
def receive_data():
    try:
        data = request.get_json()
        user_id = data["user_id"]
        data_id = data["data_id"]
        periodo = data["periodo"]

        # Insertar en la tabla registros
        cursor.execute(
            "INSERT INTO registros (user_id, data_id, periodo) VALUES (%s, %s, %s)",
            (user_id, data_id, periodo)
        )
        conn.commit()

        return jsonify({"status": "ok"}), 200
    except Exception as e:
        return jsonify({"error": str(e)}), 400

if __name__ == "__main__":
    app.run(host="127.0.0.1", port=9000)
