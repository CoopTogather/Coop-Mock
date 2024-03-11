pub mod endpoints;

pub trait InsertCommandModel<TActiveModel> {
    fn to_entity_model(self) -> TActiveModel;
}

pub trait UpdateCommandModel<TActiveModel, TUpdateDto> {
    fn set_update_active_model(&self, active_model: TActiveModel) -> TActiveModel;
}
