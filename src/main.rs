use std::error::Error;

use rust_acr::bootstrap::DB;

fn main() -> Result<(), Box<dyn Error>> {
    let data = DB::read()?;

    println!("{:#?}", data);

    Ok(())
}
