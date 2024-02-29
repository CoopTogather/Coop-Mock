pub mod endpoints;

pub trait CommandModel<TActiveModel> {
    fn to_entity_model(self) -> TActiveModel;
}
