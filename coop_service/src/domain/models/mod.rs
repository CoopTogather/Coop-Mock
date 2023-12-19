pub mod settings;

pub trait CommandModel<TDiesel> {
    fn to_diesel_model(self) -> TDiesel;
}
