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
            Genero::Femenino => "femenino".to_string(),
            Genero::Masculino => "masculino".to_string(),
            Genero::Joselito => "joselito".to_string(),
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
    pub dni: i32,
    pub password: String,
    pub email: String,
}

#[derive(Debug)]
pub struct NewUser{
    pub nombre: String,
    pub edad: i32,
    pub genero: Genero,
    pub grado: i32,
    pub dni: i32,
    pub password: String,
    pub email: String,
}


#[derive(Debug)]
pub struct UsersList{
    //A este struct estoy pensando agregarle metodos como de "analisis" sobre sus propios campos, ej: para saber cuantos femeninos y masculones hay 
    pub usuarios: Vec<UserDB>,
    pub cantidad_usuarios: usize,
}

pub async fn insert_user(usuario: NewUser, pool: &Pool) -> Result<UserDB, DbError>{
    //Esta funcion inserta un usuario en la base de datos y retorna un UserDB uwu

    let cliente = pool.get().await?;

    let row = cliente
        .query_one("INSERT INTO users (nombre, edad, genero, grado, dni, password, email)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, nombre, edad, genero, grado, dni, password, email", 
        &[
            &usuario.nombre,
            &usuario.edad,
            &usuario.genero.to_string(),
            &usuario.grado,
            &usuario.dni,
            &usuario.password,
            &usuario.email
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


pub async fn delete_user(user_id: i32, pool: &Pool) -> Result<bool, DbError>{
    let cliente = pool.get().await?;

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


pub async fn get_user(user_id: i32, pool: &Pool) -> Result<UserDB, DbError>{
    let cliente = pool.get().await?;

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


pub async fn get_all_users(pool: &Pool) -> Result<UsersList, DbError>{
    let client = pool.get().await?;

    let mut vector_final = Vec::new();

    let rows = client
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