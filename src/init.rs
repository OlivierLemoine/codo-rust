use crate::diesel::Connection;

pub fn diesel_init() -> diesel::MysqlConnection {
    diesel::MysqlConnection::establish("mysql://diesel:@localhost:3306/TodoDb").unwrap()
}