use bcrypt::{DEFAULT_COST,BcryptError};
use rand::prelude::*;

lazy_static::lazy_static!{
    pub static ref SALT : [u8; 16] =[
        82,10,210,43,67,213,76,36,56,190,71,13,44,20,204,198,
    ];
}

pub fn hash_password(password:&str)->Result<String,BcryptError>{
    match bcrypt::hash_with_salt(password,DEFAULT_COST,*SALT){
        Ok(result)=> Ok(result.to_string()),
        Err(BcryptError::CostNotAllowed(cost)) => {
            Err(BcryptError::CostNotAllowed(cost))
        }
        Err(_) => panic!("Unexpected Bcrypt error."),
    }
}
pub fn verify_password(password:&str,hash:&str)->Result<bool,BcryptError>{
    match bcrypt::verify(password, hash) {
        Ok(bool) => Ok(bool),
        Err(BcryptError::InvalidCost(_))
        | Err(BcryptError::InvalidPrefix(_))
        | Err(BcryptError::InvalidHash(_))
        | Err(BcryptError::InvalidBase64(_))=>{
            Err(BcryptError::InvalidHash(hash.to_string()))
        }
        Err(_) => panic!("Unexpected Bcrypt error."),
    }
}

pub fn generate_id()->u32{
    let mut rng = rand::thread_rng();
    rng.gen::<u32>()
}