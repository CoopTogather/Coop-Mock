pub mod endpoints;

pub trait CommandModel<TActiveModel> {
    fn to_active_model(self) -> TActiveModel;
}
