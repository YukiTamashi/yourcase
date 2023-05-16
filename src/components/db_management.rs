use serde::Serialize;

#[derive(Serialize)]
pub struct Store{
    name: String
}

#[derive(Clone, Copy)]
enum Management{
    Main,
    NewStore,
    NewPromoter,
    NewPurchase,
    NewModel,
}