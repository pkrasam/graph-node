use graph::components::metrics::{Collector, Counter, Gauge, Opts, PrometheusError};
use graph::prelude::MetricsRegistry as MetricsRegistryTrait;

use std::collections::HashMap;

pub struct MockMetricsRegistry {}

impl MockMetricsRegistry {
    pub fn new() -> Self {
        Self {}
    }
}

impl Clone for MockMetricsRegistry {
    fn clone(&self) -> Self {
        Self {}
    }
}

impl MetricsRegistryTrait for MockMetricsRegistry {
    fn register(&self, _name: &str, _c: Box<dyn Collector>) {
        // Ignore, we do not register metrics
    }

    fn global_counter(
        &self,
        name: &str,
        help: &str,
        subgraph: Option<&str>,
    ) -> Result<Counter, PrometheusError> {
        let mut const_labels = HashMap::new();
        if let Some(subgraph) = subgraph {
            const_labels.insert(String::from("subgraph"), String::from(subgraph));
        }
        let opts = Opts::new(name, help).const_labels(const_labels);
        Counter::with_opts(opts)
    }

    fn global_gauge(&self, name: &str, help: &str) -> Result<Gauge, PrometheusError> {
        let opts = Opts::new(name, help);
        Gauge::with_opts(opts)
    }

    fn unregister(&self, _: Box<dyn Collector>) {
        return;
    }
}
