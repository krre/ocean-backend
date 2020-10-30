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

pub mod types {
    pub type Id = i32;

    #[derive(Clone, PartialEq)]
    pub enum UserCode {
        Admin,
        User,
        Conspirator,
        Fierce,
    }

    #[derive(Clone)]
    pub struct User {
        pub id: Id,
        pub code: UserCode,
    }
}
