// mod front_of_house {
//    pub mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//    pub mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }

//     pub fn eat_at_restaurant() {
//         // Absolute path
//         crate::front_of_house::hosting::add_to_waitlist();
    
//         // Relative path
//         front_of_house::hosting::add_to_waitlist();
//     }

//     fn serve_order() {}

//     mod back_of_house {
//         fn fix_incorrect_order() {
//             cook_order();
//             super::serve_order();
//         }
//         fn cook_order() {}
//     }
// }

pub fn hit_and_run() {
    println!("{}", "The nao lai nhu vay, cach 2 rank");
}


