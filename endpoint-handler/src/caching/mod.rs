use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    endpoint_template::{Template, TemplateImpl},
    utils::path::first_scope,
};

pub struct TemplateCachingImpl {
    pub templates: Arc<RwLock<HashMap<String, Vec<TemplateImpl>>>>,
}

/// Trait for template caching.
pub trait TemplateCaching: Sync + Send {
    /// Retrieves a vector of templates for a given scope.
    ///
    /// # Arguments
    ///
    /// * `first_scope` - The first scope to filter the templates.
    ///
    /// # Returns
    ///
    /// An optional vector of `TemplateImpl` instances.
    fn get_template_vec(&self, first_scope: &str) -> Option<Vec<TemplateImpl>>;

    /// Adds a vector of templates for a given scope.
    ///
    /// # Arguments
    ///
    /// * `first_scope` - The first scope to associate the templates with.
    /// * `templates` - The vector of `TemplateImpl` instances to add.
    fn add_template_vec(&self, first_scope: &str, templates: Vec<TemplateImpl>);

    /// Finds a template by its path and method.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the template.
    /// * `method` - The HTTP method of the template.
    ///
    /// # Returns
    ///
    /// An optional `TemplateImpl` instance.
    fn find_template(&self, path: &str, method: &str) -> Option<TemplateImpl>;
}

impl Default for TemplateCachingImpl {
    fn default() -> Self {
        Self {
            templates: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl TemplateCaching for TemplateCachingImpl {
    fn get_template_vec(&self, first_scope: &str) -> Option<Vec<TemplateImpl>> {
        let template_hash = self.templates.read().unwrap();

        match template_hash.get(first_scope).cloned() {
            Some(templates) => Some(templates),
            None => None,
        }
    }

    fn add_template_vec(&self, first_scope: &str, templates: Vec<TemplateImpl>) {
        let mut template_hash = self.templates.write().unwrap();

        template_hash.insert(first_scope.to_string(), templates);
    }

    fn find_template(&self, path: &str, method: &str) -> Option<TemplateImpl> {
        let first_scope = first_scope(path);

        if first_scope.is_none() {
            return None;
        }

        let first_scope = first_scope.unwrap();

        let template_vec = self.get_template_vec(first_scope);

        if template_vec.is_none() {
            return None;
        }

        let template_vec = template_vec.unwrap();

        for template in template_vec {
            if template.matches(path, method) {
                return Some(template);
            }
        }

        None
    }
}
