#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate diesel;

pub mod api;
pub mod app;
pub mod config;
pub mod controller;
pub mod db;
pub mod json_rpc;
pub mod model;
pub mod telegram_bot;
pub mod watchdog;

pub mod types {
    pub type Id = i32;

    #[derive(Clone, Debug, PartialEq)]
    pub enum UserCode {
        Admin,
        User,
        Anonym,
    }

    #[derive(Clone)]
    pub struct User {
        pub id: Id,
        pub code: UserCode,
        pub name: String,
    }
}
