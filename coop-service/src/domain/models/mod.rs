pub mod endpoints;

pub trait CommandModel<TDiesel> {
    fn to_active_model(self) -> TDiesel;
}
