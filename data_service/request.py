import requests

url = "http://127.0.0.1:9000/"
data = {
    "user_id": 456,
    "data_id": "XYZ789",
    "periodo": "2025-Q2"
}

response = requests.post(url, json=data)
print(response.json())