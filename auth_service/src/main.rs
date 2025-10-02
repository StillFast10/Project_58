use actix_web::{get, post, web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use deadpool_postgres::{Config as PoolConfig, Pool, ManagerConfig, RecyclingMethod, Client };
use tokio_postgres::NoTls;
//Uso mi libreria de sabre como provisional
use sabre::database::{self, delete_chat, get_chat_by_id_and_user_id, get_table_columns, get_user_by_email, insert_chat, insert_user, setup_database, User};
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
    println!("Login request from: {:?}", req.peer_addr());

    match db.get().await {
        Ok(db_client) => {
            match get_user_by_email(&db_client, &body.email).await {
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

    let pool = cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls).unwrap();

    if let Err(e) = setup_database(&pool).await {
        eprintln!("Error al configurar la base de datos: {}", e);
        std::process::exit(1);
    }
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive())
            .service(login)
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}