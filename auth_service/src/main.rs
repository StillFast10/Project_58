use std::str::FromStr;

use actix_web::{get, post, web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use deadpool_postgres::{Config as PoolConfig, Pool, ManagerConfig, RecyclingMethod, Client };
use tokio_postgres::NoTls;
//Uso mi libreria de sabre como provisional
use db_lib::users_db::{self, delete_user, get_user_by_email, NewUser};
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
    nombre: String,
    apellidos: String,
    dni: i64,
    grado: i32,
    genero: String, 
    password: String,
    email: String,
    edad: i32,
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


#[post("/signup")]
async fn signup(req: HttpRequest, body: web::Json<SignupRequest>, db: web::Data<Pool>,) -> impl Responder{

    println!("Signup enter");

    let db_client = match db.get().await{
        Ok(x) => {x},

        Err(e) => {
            let respuesta = HttpResponse::InternalServerError()
            .json(serde_json::json!({
                "message": "Error de base de datos aaa"
            }));

            return respuesta;
        }
    };

    match users_db::comprobar_email(&body.email, &db_client).await{
        Ok(x) => {
            if x == true{
                let respuesta = HttpResponse::Conflict()
                .json(serde_json::json!({
                    "message": "El email ya existe :("
                }));

                return respuesta;
            }

        },

        Err(e) => {
            let respuesta = HttpResponse::InternalServerError()
            .json(serde_json::json!({
                "message": "Error de base de datos email"
            }));

            return respuesta;
        },
    }

    match users_db::comprobar_dni(body.dni, &db_client).await {
        Ok(x) => {
            if x == true{
                let respuesta = HttpResponse::Conflict()
                .json(serde_json::json!({
                    "message": "El dni ya existe :("
                }));

                return respuesta;
            }

        },

        Err(e) => {
            let respuesta = HttpResponse::InternalServerError()
            .json(serde_json::json!({
                "message": "Error de base de datos dni"
            }));

            return respuesta;
        },
    }


    println!("Genero llegado: {}", &body.genero);

    let genero = users_db::Genero::from_str(&body.genero);

    //Hagamos el usuario uwucs

    let usuario = NewUser{
        nombres: body.nombre.clone(),
        apellidos: body.apellidos.clone(),
        edad: body.edad,
        genero: genero.unwrap(),
        grado: body.grado,
        dni: body.dni,
        password: body.password.clone(),
        email: body.email.clone(),
    };

    match users_db::insert_user(usuario, &db_client).await{
        Ok(x) => {
            let respuesta = HttpResponse::Ok()
            .json(serde_json::json!({
                "message": "Cuenta creada uwu"
            }));

            return respuesta;
        },

        Err(e) => {
            let respuesta = HttpResponse::InternalServerError()
            .json(serde_json::json!({
                "message": "Error de base de datos final"
            }));

            return respuesta;
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Auth Service");
    //Aca creariamos la config de la base de datos cfg

    let mut cfg = PoolConfig::new();

    cfg.dbname = Some("project_58_2".to_string());
    cfg.user = Some("rusty".to_string());
    cfg.password = Some("tototo".to_string());
    cfg.host = Some("localhost".to_string());

    let pool = cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls).unwrap();


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive())
            .service(login)
            .service(signup)
    })
    .bind(("127.0.0.1", 9002))?
    .run()
    .await
}