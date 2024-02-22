pub struct Lookup {
    domain: 
}

pub struct BatchLookup {

}

pub struct LookupConfig {

}

pub type RdResult = Result<Record, Error>;

trait Packet {
    pub fn bytes() -> ;
    pub fn vec() -> Vec<u8>;

    /**
     * Execute the query
     */
    pub fn execute() -> RdResult;
}

impl Lookup {
    pub fn packet() -> Option<impl Packet>{
        
    }
}

#[macro_export]
macro_rules! lookup {
    () => { ... };
    ($elem:expr) => { 
        Lookup {
            domain: $elem,
        }
    };
    ($($x:expr),+ $(,)?) => { 
        
    };
}

#[test]
fn test_lookup_macro_domain() {
    assert_eq!("radyns.borefunc.dev", lookup!("radyns.borefunc.dev").domain)
}

mod raw;