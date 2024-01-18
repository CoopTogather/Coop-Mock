#[cfg(test)]
mod caching_tests {
    use std::ops::Deref;

    use endpoint_handler::{
        caching::{TemplateCaching, TemplatesCachingImpl},
        endpoint_template::TemplateImpl,
    };

    #[test]
    pub fn create_new_cache() {
        let template_cache = TemplatesCachingImpl::default();
        let templates = template_cache.templates.deref();

        assert_eq!(templates.read().unwrap().len(), 0);
    }

    #[test]
    pub fn insert_template_into_cache() {
        let template_cache = TemplatesCachingImpl::default();
        let mock_template_1 = TemplateImpl::new("/test".to_string(), "GET".to_string(), None);
        let mock_template_2 = TemplateImpl::new("/test".to_string(), "POST".to_string(), None);
        let mock_template_3 = TemplateImpl::new("/test".to_string(), "PUT".to_string(), None);

        template_cache.add_range(
            "test",
            vec![mock_template_1, mock_template_2, mock_template_3],
        );

        let templates = template_cache.get("test").unwrap();

        assert_eq!(templates.len(), 3);
    }
}
