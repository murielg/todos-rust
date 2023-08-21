-- use only in development // comment app to keep --
DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

-- use only in development --
CREATE USER app_user PASSWORD 'app_pwd_2_change';
CREATE DATABASE app_db owner app_user ENCODING = 'UTF-8';
