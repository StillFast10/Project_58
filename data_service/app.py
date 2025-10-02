from flask import Flask, request

app = Flask(__name__)
#si ves esto que metodo uso aca y
#que barra le pongo "/"
@app.route('/', methods=['POST'])
def receive_data():
    data = request.json
    user_id = data.get('user_id') if data else None
    print(f"recibido {user_id}")
    
    return "OK", 200

if __name__ == '__main__':
    app.run(host="127.0.0.1", port=9000)