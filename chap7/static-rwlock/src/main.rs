use lazy_static::lazy_static;
use std::collections::HashSet;
use std::error::Error;
use std::sync::RwLock;

lazy_static! {
    pub static ref DOGS: RwLock<HashSet<&'static str>> = {
        let dogs = ["紫", "トイプードル"].iter().cloned().collect();
        RwLock::new(dogs)
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    {
        let dogs = DOGS.read()?;
        assert!(dogs.contains("紫"));
        assert!(dogs.contains("トイプードル"));
    }

    fn stringify(x: impl ToString) -> String { x.to_string() };

    DOGS.write()?.insert("ブル・テリア");

    std::thread::spawn(move || {
        DOGS.write().map(|mut ds| ds.insert("コーギー")).map_err(stringify)
    }).join().expect("Thread error")?;

    assert!(DOGS.read().map_err(stringify)?.contains("ブル・テリア"));
    assert!(DOGS.read().map_err(stringify)?.contains("コーギー"));

    Ok(())
}
