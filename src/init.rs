use dotenv::dotenv;
use util_error::ErrorKind;

pub async fn init() -> Result<(), ErrorKind> {
    // env
    dotenv().ok();

    // init log
    log4rs::init_file("log4rs.yml", Default::default())?;
    
    Ok(())
}
