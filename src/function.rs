extern crate bcrypt;
extern crate mysql;

use super::utils;
use mysql::*;
use mysql::prelude::*;



#[derive(Debug, PartialEq, Eq)]
struct User {
    id:u32,
    password:String,
}

#[derive(Debug)]
pub(crate) struct Conn {
    mysql:PooledConn,
}

impl Conn{
    pub fn new(database_url:&str)->Result<Conn>{
        let mut mysql = connection(database_url).unwrap();
        creat_table(&mut mysql).expect("creat_table error");
        Ok(Conn{mysql})
    }
    pub fn login(&mut self,id:u32,password:&str)->std::result::Result<bool,String>{
        match self.select(id) {
            Some(user) => {
                match utils::verify_password(password,&user.password) {
                    Ok(result)=> Ok(result),
                    Err(e)=> Err(format!("verify_password error:{}",e)),
                }
            }
            None => Err("id is not exist".to_string()),
        }
    }
    pub fn remove(&mut self,id:u32)->Result<u64>{
        let sql = format!("delete from id_password where id = {} ",id);
        let result = sql.run(&mut self.mysql);
        match result{
            Ok(res) =>{
                println!("remove success");
                Ok(res.affected_rows())
            }
            Err(e)=>{
                println!("remove failed");
                Err(e)
            }
        }
    }
    /// return your id
    pub fn registered(&mut self, password:&str) ->Result<u32>{
        let hash_pwd = utils::hash_password(password).unwrap();
        let id = utils::generate_id();
        let sql = format!("insert into id_password(id,password) values({},'{}')",id,hash_pwd);
        let result = sql.run(&mut self.mysql);
        match result{
            Ok(res) =>{
                println!("insert success");
                Ok(id)
            }
            Err(e)=>{
                println!("insert failed");
                Err(e)
            }
        }
    }
    pub fn update(&mut self,id:u32,password:&str)->Result<u64>{
        let hash_pwd = utils::hash_password(password).unwrap();
        let sql = format!("update id_password set password = '{}' where id = {}",hash_pwd,id);
        let result = sql.run(&mut self.mysql);
        match result{
            Ok(res) =>{
                println!("update success");
                Ok(res.affected_rows())
            }
            Err(e)=>{
                println!("update failed");
                Err(e)
            }
        }
    }
    fn select(&mut self,id:u32) -> Option<User> {
        let users = self.mysql
            .exec_first("SELECT * FROM id_password WHERE id = :id",
                        params! {
                        "id" => id,
                    },
            )
            .map(
                |row| {
                    row.map(|(id, password)| User {
                        id,
                        password,
                    })
                }
            );
        match users {
            Ok(Some(user)) => Some(user),
            _ => None,
        }
    }
}



fn connection(database_url:&str)->Result<PooledConn>{
    let pool = Pool::new(database_url)?;
    let mut mysql = pool.get_conn()?;
    Ok(mysql)
}

fn creat_table(mysql:&mut mysql::PooledConn)->Result<()>{
    let sql = "CREATE TABLE IF NOT EXISTS id_password (
        id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT ,
        password VARCHAR(255) NOT NULL,
        PRIMARY KEY (id)
    )";
    mysql.query_drop(sql)
}

