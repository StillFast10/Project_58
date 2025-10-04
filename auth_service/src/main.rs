use actix_web::{get, post, web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use deadpool_postgres::{Config as PoolConfig, Pool, ManagerConfig, RecyclingMethod, Client };
use tokio_postgres::NoTls;
//Uso mi libreria de sabre como provisional
use db_lib::users_db::{get_user_by_email, delete_user};
use db_lib::pool;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use actix_web::cookie::{Cookie, SameSite};

#[derive(Deserialize)]
struct LoginRequest{
    email: String,
    password: String,
}


#[derive(Deserialize)]
struct SignupRequest{
    username: String,
    password: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

const SECRET0: &[u8] = b"LAPINGADEDIONISIO";


#[post("/login")]
async fn login(
    req: HttpRequest,
    body: web::Json<LoginRequest>,
    db: web::Data<Pool>,
) -> impl Responder {

    println!("AUTH SERVICE");
    println!("Login request from: {:?}", req.peer_addr());

    match db.get().await {
        Ok(db_client) => {
            match get_user_by_email(&body.email, &db_client).await {
                Ok(usuario) => {
                    if usuario.password == body.password {
                        let expiration = Utc::now()
                            .checked_add_signed(Duration::hours(24))
                            .unwrap()
                            .timestamp() as usize;

                        let claims = Claims {
                            sub: usuario.id.to_string(),
                            exp: expiration,
                        };

                        let token = encode(
                            &Header::default(),
                            &claims,
                            &EncodingKey::from_secret(SECRET0.as_ref()),
                        )
                        .unwrap();

                        
                        let cookie = Cookie::build("auth_token", token)
                            .path("/")
                            .http_only(true)
                            .same_site(SameSite::Strict)
                            .finish();

                        return HttpResponse::Ok()
                            .cookie(cookie)
                            .json(serde_json::json!({
                                "message": "Login correcto"
                            }));
                    } else {
                        return HttpResponse::Unauthorized().json(serde_json::json!({
                            "message": "Credenciales inválidas"
                        }));
                    }
                }

                Err(_) => {
                    return HttpResponse::Unauthorized().json(serde_json::json!({
                        "message": "Usuario no encontrado"
                    }));
                }
            }
        }

        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "No se pudo conectar a la base de datos"
            }));
        }
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Auth Service");
    //Aca creariamos la config de la base de datos cfg

    let pool = pool::DbConfig{
        db_name: "project_58_2".to_string(),
        user: "rusty".to_string(),
        password: "tototo".to_string(),
        host: "localhost".to_string(),
    };
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive())
            .service(login)
    })
    .bind(("127.0.0.1", 9002))?
    .run()
    .await
}