pub mod db;
pub mod gatherer;
pub mod structs;
pub mod utils;

use dotenv::dotenv;
use std::{cell::RefCell, rc::Rc};
use sysinfo::SystemExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let sys = Rc::from(RefCell::from(gatherer::create_sys()));

    let mut i = 0;

    loop {
        let mut system = sys.try_borrow_mut().unwrap();
        system.refresh_all();

        let data = gatherer::sys_info_gatherer(system);

        i += 1;

        if i == 20 {
            match db::insert_into_influx(data, "test", "test").await {
                Ok(response) => response,
                Err(_) => {
                    panic!("PANIC")
                }
            };
            println!("DONE");
            break;
        }
    }

    Ok(())
}
