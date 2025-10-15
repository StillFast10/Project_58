use std::str::FromStr;

use tokio_postgres::{NoTls};
use deadpool_postgres::{Client, Config, Pool};
use serde::Serialize;
use crate::error::DbError;

//Este sale de la db
//password estara hasheada btw jiji

#[derive(Debug)]
pub enum Genero{
    Femenino,
    Masculino,
    Joselito
}

impl FromStr for Genero{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "femenino" => Ok(Genero::Femenino),
            "masculino" => Ok(Genero::Masculino),
            "joselito" => Ok(Genero::Joselito),
            _ => Err(format!("Valor inválido para genero: {}", s)),
        }
    }

}

impl ToString for Genero {
    fn to_string(&self) -> String {
        match self {
            Genero::Femenino => "Femenino".to_string(),
            Genero::Masculino => "Masculino".to_string(),
            Genero::Joselito => "Joselito".to_string(),
        }
    }
}



#[derive(Debug)]
pub struct UserDB{
    pub id: i32,
    pub nombre: String,
    pub edad: i32,
    pub genero: Genero,
    pub grado: i32,
    pub dni: i64,
    pub password: String,
    pub email: String,
}

#[derive(Debug)]
pub struct NewUser{
    pub nombres: String,
    pub apellidos: String,
    pub edad: i32,
    pub genero: Genero,
    pub grado: i32,
    pub dni: i64,
    pub password: String,
    pub email: String,
}


#[derive(Debug)]
pub struct UsersList{
    //A este struct estoy pensando agregarle metodos como de "analisis" sobre sus propios campos, ej: para saber cuantos femeninos y masculones hay 
    pub usuarios: Vec<UserDB>,
    pub cantidad_usuarios: usize,
}

pub async fn comprobar_email(email: &str, cliente: &Client) -> Result<bool, DbError>{
    let stmt = "SELECT 1 FROM users WHERE email = $1 LIMIT 1";
    let row = cliente.query_opt(stmt, &[&email]).await?;

    Ok(row.is_some())
}

pub async fn comprobar_dni(dni: i64, cliente: &Client) -> Result<bool, DbError>{
    let stmt = "SELECT 1 FROM users WHERE dni = $1 LIMIT 1";
    let row = cliente.query_opt(stmt, &[&dni]).await?;

    Ok(row.is_some())
}


pub async fn insert_user(usuario: NewUser, cliente: &Client) -> Result<UserDB, DbError>{
    //Esta funcion inserta un usuario en la base de datos y retorna un UserDB uwu

    let row = cliente
        .query_one("INSERT INTO users (nombre, edad, genero, grado, dni, password, email, apellidos)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, nombre, edad, genero, grado, dni, password, email", 
        &[
            &usuario.nombres,
            &usuario.edad,
            &usuario.genero.to_string(),
            &usuario.grado,
            &usuario.dni,
            &usuario.password,
            &usuario.email,
            &usuario.apellidos
        ]
    ).await?;


    let genero: String = row.get("genero");
    let genero = Genero::from_str(&genero)
        .map_err(|e| DbError::Conversion(e))?;

    Ok(UserDB{
        id: row.get("id"),
        nombre: row.get("nombre"),
        edad: row.get("edad"),
        genero: genero,
        grado: row.get("grado"),
        dni: row.get("dni"),
        password: row.get("password"),
        email: row.get("email"),
    })
}


pub async fn delete_user(user_id: i32, cliente: &Client) -> Result<bool, DbError>{

    let rows_afectadas = cliente
        .execute("DELETE FROM users WHERE id = $1", &[&user_id])
        .await?;

    if rows_afectadas > 0{
        Ok(true)
    }
    else {
        Ok(false)
    }
}


pub async fn get_user_by_email(email: &str, cliente: &Client) -> Result<UserDB, DbError>{
    let row = cliente
        .query_one("SELECT id, nombre, edad, genero, grado, dni, password, email FROM users WHERE email = $1", &[&email])
        .await?;


    let genero = row.get("genero");
    let genero = Genero::from_str(genero)
        .map_err(|e| DbError::Conversion(e))?;

    Ok(UserDB {
        id: row.get("id"),
        nombre: row.get("nombre"),
        edad: row.get("edad"),
        genero: genero,
        grado: row.get("grado"),
        dni: row.get("dni"),
        password: row.get("password"),
        email: row.get("email"),
    })
}

pub async fn get_user(user_id: i32, cliente: &Client) -> Result<UserDB, DbError>{

    let row = cliente
        .query_one("SELECT id, nombre, edad, genero, grado, dni, password, email FROM users WHERE id = $1", &[&user_id])
        .await?;

    let genero = row.get("genero");
    let genero = Genero::from_str(genero)
        .map_err(|e| DbError::Conversion(e))?;

    Ok(UserDB {
        id: row.get("id"),
        nombre: row.get("nombre"),
        edad: row.get("edad"),
        genero: genero,
        grado: row.get("grado"),
        dni: row.get("dni"),
        password: row.get("password"),
        email: row.get("email"),
    })
}


pub async fn get_all_users(cliente: &Client) -> Result<UsersList, DbError>{
    let mut vector_final = Vec::new();

    let rows = cliente
        .query("SELECT id, nombre, edad, genero, grado, dni, password, email FROM users", &[])
        .await?;

    for row in rows{
        let genero_str: String = row.get("genero");
        let genero = Genero::from_str(&genero_str)
            .map_err(|e| DbError::Conversion(e))?;

        let user = UserDB{
            id: row.get("id"),
            nombre: row.get("nombre"),
            edad: row.get("edad"),
            genero,
            grado: row.get("grado"),
            dni: row.get("dni"),
            password: row.get("password"),
            email: row.get("email"),          
        };

        vector_final.push(user);
    }

    let len = vector_final.len();

    Ok(UsersList{
        usuarios: vector_final,
        cantidad_usuarios: len,
    })
}