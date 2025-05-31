-- Add migration script here
alter table users rename password to password_hash;