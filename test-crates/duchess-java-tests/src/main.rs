#![allow(dead_code)]

use duchess::prelude::*;

duchess::java_package! {
    package take_null;

    public class TakeNull { * }
}

fn main() -> duchess::Result<()> {
    println!("My Duchess test");
    let date = java::util::Date::new().execute()?;
    println!("My Duchess test");
    let hash_code = date
        .equals(duchess::Null)
        .execute()?;
    Ok(())
}
