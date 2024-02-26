#[cfg(test)]
mod caching_tests {
    use endpoint_handler::{
        caching::{TemplateCaching, TemplateCachingImpl},
        endpoint_template::TemplateImpl,
    };
    use std::ops::Deref;

    #[test]
    pub fn create_new_cache() {
        let template_cache = TemplateCachingImpl::default();
        let templates = template_cache.templates.deref();

        assert_eq!(templates.read().unwrap().len(), 0);
    }

    #[test]
    pub fn insert_template_into_cache() {
        let template_cache = TemplateCachingImpl::default();
        let mock_template_1 = TemplateImpl::new("/test".to_string(), "GET".to_string(), None);
        let mock_template_2 = TemplateImpl::new("/test".to_string(), "POST".to_string(), None);
        let mock_template_3 = TemplateImpl::new("/test".to_string(), "PUT".to_string(), None);

        let mock_template_other = TemplateImpl::new("/other".to_string(), "GET".to_string(), None);
        let mock_template_other_2 =
            TemplateImpl::new("/other".to_string(), "POST".to_string(), None);

        template_cache.add_range(
            "test",
            vec![mock_template_1, mock_template_2, mock_template_3],
        );
        template_cache.add_range("other", vec![mock_template_other, mock_template_other_2]);

        let templates = template_cache.get("test").unwrap();
        let other_templates = template_cache.get("other").unwrap();

        assert_eq!(templates.len(), 3);
        assert_eq!(other_templates.len(), 2);
    }
}
