use core::hash::Hash;
use std::collections::HashMap;

use bon::bon;
use prometheus::{
    Counter,
    Opts,
    Registry,
};

use crate::{
    error::{
        BackendError,
        Result,
    },
    metrics::traits::{
        ImageMetrics,
        SoftwareMetrics,
    },
};

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash, strum::Display)]
pub enum CounterName
{
    SoftwareCreationFailure,
    SoftwareCreationSuccess,
    ImageCreationFailure,
    ImageCreationSuccess,
}

// TODO actually provide implementations of all of this.

#[derive(Clone, Debug)]
pub struct Prometheus
{
    counters: HashMap<CounterName, Counter>,
    registry: prometheus::Registry,
}

#[bon]
impl Prometheus
{
    #[builder]
    pub fn new(#[builder(field)] counter_opts: HashMap<CounterName, Opts>) -> Result<Self>
    {
        let mut counters = HashMap::new();
        let registry = Registry::new();

        for (name, counter_opt) in counter_opts
        {
            let counter = Counter::with_opts(counter_opt)?;
            registry.register(Box::new(counter.clone()))?;
            match counters.insert(name.clone(), counter)
            {
                Some(_) => tracing::warn!("Counter already exists for this counter name: {name}"),
                None => tracing::info!("New counter inserted for name: {name}"),
            }
        }
        Ok(Self { counters, registry })
    }

    pub fn registry(&self) -> &Registry { &self.registry }

    pub fn increment_counter(
        &self,
        counter_name: CounterName,
    ) -> Result<()>
    {
        let counter = self
            .counters
            .get(&counter_name)
            .ok_or(BackendError::HashMapValueMissing(counter_name.to_string()))?
            .clone();
        counter.inc();
        Ok(())
    }
}

impl<S: prometheus_builder::State> PrometheusBuilder<S>
{
    pub fn counter_opt(
        mut self,
        name: CounterName,
        value: Opts,
    ) -> Self
    {
        self.counter_opts.insert(name, value);
        self
    }
}

impl ImageMetrics for Prometheus
{
    async fn record_image_creation_failure(&self) -> Result<()>
    {
        self.increment_counter(CounterName::ImageCreationFailure)
    }

    async fn record_image_creation_success(&self) -> Result<()>
    {
        self.increment_counter(CounterName::ImageCreationSuccess)
    }
}

impl SoftwareMetrics for Prometheus
{
    async fn record_software_creation_failure(&self) {}

    async fn record_software_creation_success(&self) {}

    async fn record_get_all_software_failure(&self) {}

    async fn record_get_all_software_success(&self) {}
}
