use endpoint_handler::endpoint_template::{parameter::TemplateScopeTypesEnum, TemplateImpl};

#[test]
pub fn endpoint_path_string_convert_to_template() {
    let path = String::from("/mock/{id:string}");
    let method = String::from("POST");
    let template_path = TemplateImpl::new(path, method, None);

    assert_eq!(template_path.paths.len(), 2);

    assert_eq!(template_path.paths[0].scope, "mock");
    assert!(template_path.paths[0].parameter.is_none());

    assert_eq!(template_path.paths[1].scope, "{id:string}");
    assert!(template_path.paths[1].parameter.is_some());
    assert_eq!(
        template_path.paths[1].parameter.as_ref().unwrap().name,
        String::from("id")
    );
    assert_eq!(
        template_path.paths[1]
            .parameter
            .as_ref()
            .unwrap()
            .scope_type,
        TemplateScopeTypesEnum::String
    );
}

#[cfg(test)]
mod template_matches_tests {
    use endpoint_handler::endpoint_template::{Template, TemplateImpl};

    #[test]
    pub fn endpoint_matches_with_single_parameter() {
        let path = String::from("/mock/{id:number}");
        let method = String::from("POST");
        let template_path = TemplateImpl::new(path, method, None);

        assert_eq!(template_path.matches("/mock/1", "POST"), true);
        assert_eq!(template_path.matches("/mock/1", "GET"), false);
        assert_eq!(template_path.matches("/mock/1", "PUT"), false);
        assert_eq!(template_path.matches("/mock/1", "DELETE"), false);
        assert_eq!(template_path.matches("/mock/1", "PATCH"), false);

        assert_eq!(template_path.matches("/mock/1/list", "POST"), false);

        assert_eq!(template_path.matches("/mock/qwe", "POST"), false);
    }

    #[test]
    pub fn endpoint_matches_with_path_after_parameter() {
        let path = String::from("/mock/{id:number}/list");
        let method = String::from("POST");
        let template_path = TemplateImpl::new(path, method, None);

        assert_eq!(template_path.matches("/mock/1/list", "POST"), true);
        assert_eq!(template_path.matches("/mock/1/list", "GET"), false);
        assert_eq!(template_path.matches("/mock/1/list", "PUT"), false);
        assert_eq!(template_path.matches("/mock/1/list", "DELETE"), false);
        assert_eq!(template_path.matches("/mock/1/list", "PATCH"), false);

        assert_eq!(template_path.matches("/mock/1", "POST"), false);
        assert_eq!(template_path.matches("/mock/1/list/1", "POST"), false);
    }
}
