
DROP DATABASE
IF EXISTS TodoDb;

DROP USER IF EXISTS 'diesel'@'localhost';
CREATE USER 'diesel'@'localhost' IDENTIFIED BY '';

CREATE DATABASE TodoDb;

GRANT ALL PRIVILEGES ON TodoDb.* TO 'diesel'@'localhost';

USE TodoDb;

CREATE TABLE todos
(
    id INT PRIMARY KEY NOT NULL
    AUTO_INCREMENT,
    name CHARACTER
    (128),
    is_checked BOOLEAN
);

-- libmysqlclient-dev