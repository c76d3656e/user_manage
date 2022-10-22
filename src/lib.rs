mod utils;
mod function;
mod error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut user_manage = function::Conn::new("mysql://root:Zhongfeng!@172.16.0.2:3306/user").unwrap();
        let result = user_manage.login(1, "123456");
        println!("{:?}",result);
        let result = user_manage.remove(1);
        println!("{:?}",result);
        let result = user_manage.insert("123456");
        let id = result.unwrap();
        println!("{:?}", id);
        let result = user_manage.login(id, "123456");
        println!("{:?}",result);
        let result = user_manage.remove(id);
        println!("{:?}",result);
    }
}
