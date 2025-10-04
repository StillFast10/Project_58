use actix_web::{get,web, post, App, HttpServer, Responder, HttpRequest, HttpResponse};
use reqwest::Client;
use actix_cors::Cors;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};


const DATA_SERVICE: &str = "http://127.0.0.1:9000";
const LOGIN: &str = "http://127.0.0.1:9002";

const SECRET0: &[u8] = b"CARLITOSCARLANGASXD";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn validate_jwt_from_cookie(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    if let Some(cookie) = req.cookie("auth_token") {
        let token = cookie.value();

        match decode::<Claims>(token,&DecodingKey::from_secret(SECRET0.as_ref()),&Validation::new(Algorithm::HS256),) {
            Ok(token_data) => Ok(token_data.claims),

            Err(_) => Err(HttpResponse::Unauthorized().body("Token inválido o expirado")),
        }
    } 
    else {
        Err(HttpResponse::Unauthorized().body("Token no encontrado"))
    }
}


//Aqui van los handlers uwu

#[post("/login")]
async fn login(req: HttpRequest, body: String) -> impl Responder {
    println!("Login request from: {:?}", req.peer_addr());
    println!("Body received: {}", body);

    let client = Client::new();

    let resp = client
        .post(format!("{}/login", LOGIN))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;

    match resp {
        Ok(r) => {
            println!("Response received from Auth service");
            let status = r.status();
            let mut client_resp = HttpResponse::build(status);

            for (key, value) in r.headers().iter() {
                if key.as_str().eq_ignore_ascii_case("set-cookie") {
                    client_resp.append_header((key.clone(), value.clone()));
                }
                if key.as_str().eq_ignore_ascii_case("content-type") {
                    client_resp.append_header((key.clone(), value.clone()));
                }
            }

            let text = r.text().await.unwrap_or_else(|_| "Error reading response".to_string());
            client_resp.body(text)
        }

        Err(e) => {
            HttpResponse::InternalServerError()
                .body(format!("Error: {} al conectarse al servicio de Auth", e))
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("APIGATEWAY RUNNING ON PORT 8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
