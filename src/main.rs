pub mod db;
pub mod gatherer;
pub mod structs;
pub mod utils;


use std::{cell::RefCell, rc::Rc};
use sysinfo::SystemExt;
use dotenv::dotenv;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let sys = Rc::from(RefCell::from(gatherer::create_sys()));

    let mut i = 0;

    loop {
        let mut system = sys.try_borrow_mut().unwrap();
        system.refresh_all();

        let data = gatherer::sys_info_gatherer(system);

        if i == 10 {
            break;
        }

        match db::insert_into_influx(data, "test", "test").await {
            Ok(response) => response,
            Err(_) => {
                panic!("PANIC")
            }
        };

        println!("Run iteration: {}", i);

        i += 1;
    }

    Ok(())
}
