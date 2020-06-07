mod application;
mod controller;
mod entity;
mod usecase;
mod repository;
fn main() {
    application::server::server()
}