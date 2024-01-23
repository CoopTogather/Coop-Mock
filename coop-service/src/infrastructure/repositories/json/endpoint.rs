use crate::{
    domain::{models::endpoints::EndpointDto, repositories::endpoint::EndpointRepository},
    infrastructure::json_handlers::reader::read_json_file,
};

pub struct EndpointJsonRepositoryImpl {}

#[async_trait::async_trait]
impl EndpointRepository for EndpointJsonRepositoryImpl {
    async fn create_mock(
        &self,
        add_endpoint: crate::domain::models::endpoints::CreateEndpointDto,
    ) -> Result<(), &str> {
        let path = std::env::current_dir().expect("Unable to get current directory");
        let json_path = format!("{}/mocks/db.json", path.display());

        let read_mocks = match read_json_file::<EndpointDto>(json_path.as_str()) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        

        todo!()
    }

    async fn get_mock(
        &self,
        _endpoint_id: i32,
    ) -> Result<Option<crate::domain::models::endpoints::EndpointDto>, &str> {
        unimplemented!()
    }

    async fn get_mocks(&self) -> Result<Vec<crate::domain::models::endpoints::EndpointDto>, &str> {
        unimplemented!()
    }
}
