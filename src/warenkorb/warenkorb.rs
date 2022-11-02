use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Warenkorb {
    pub amount_apples : u32,
    pub amount_bananas : u32,
    pub amount_potatoes : u32,
}
#[derive(Serialize,Deserialize)]
pub struct ResponseStruct{
    pub amount : u32,
}
//cur {apfel : amount : 1}
//
pub struct JsonWarenkorb {
    pub apfel : Apfel,

}


pub struct Apfel{
    pub amount : u32,
    pub id : u32,
}
pub struct Banane{
    pub amount : u32,
    pub id : u32,
}
pub struct Kartoffel{
    pub amount : u32,
    pub id : u32,
}

impl Warenkorb {
    pub fn new() -> Self {
        Warenkorb{
            amount_apples: 0,
            amount_bananas : 0,
            amount_potatoes: 0,
        }
    }
}

