use std::sync::Arc;

struct Inner {
    version: Option<String>,
    layout: Box<dyn Fn(String) -> String + Send + Sync>,
}

#[derive(Clone)]
pub struct InertiaConfig {
    inner: Arc<Inner>,
}

impl InertiaConfig {
    /// Constructs a new InertiaConfig object.
    ///
    /// `layout` provides information about how to render the initial
    /// page load. See the [crate::vite] module for an implementation
    /// of this for vite.
    pub fn new(
        version: Option<String>,
        layout: Box<dyn Fn(String) -> String + Send + Sync>,
    ) -> InertiaConfig {
        let inner = Inner { version, layout };
        InertiaConfig {
            inner: Arc::new(inner),
        }
    }

    /// Returns a cloned optional version string.
    pub fn version(&self) -> Option<String> {
        self.inner.version.clone()
    }

    /// Returns a reference to the layout function.
    pub fn layout(&self) -> &(dyn Fn(String) -> String + Send + Sync) {
        &self.inner.layout
    }
}
