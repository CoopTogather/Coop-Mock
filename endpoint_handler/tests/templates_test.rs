use endpoint_handler::endpoint_template::{Template, TemplateImpl};

#[test]
pub fn endpoint_path_string_convert_to_template() {
    let path = String::from("/mock/{id:u32}");
    let method = String::from("POST");
    let template_path = TemplateImpl::new(path, method, None);

    assert_eq!(template_path.paths.len(), 2);

    assert_eq!(template_path.paths[0].scope, "mock");
    assert_eq!(template_path.paths[0].is_parameter, false);
    assert_eq!(template_path.paths[0].parameter_name, None);

    assert_eq!(template_path.paths[1].scope, "id");
    assert_eq!(template_path.paths[1].is_parameter, true);
    assert_eq!(
        template_path.paths[1].parameter_name,
        Some(String::from("{id:u32}"))
    );
    assert_eq!(
        template_path.paths[1].parameter_type,
        Some(String::from("u32"))
    );
}

#[cfg(test)]
mod template_matches_tests {
    use endpoint_handler::endpoint_template::{Template, TemplateImpl};

    #[test]
    pub fn endpoint_matches_with_single_parameter() {
        let path = String::from("/mock/{id:u32}");
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
        let path = String::from("/mock/{id:u32}/list");
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
