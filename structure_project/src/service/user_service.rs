#![allow(dead_code)]
use crate::service::clazz_service as clazz;

pub fn create() {

    // create user info
    println!("{}", "Create user info ");
    // call clazz service for update clazz
    println!("{}", "start call clazz service");
    clazz::create();
    println!("{}", "end call clazz service");
}