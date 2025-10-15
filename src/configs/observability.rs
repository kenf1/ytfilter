use opentelemetry::KeyValue;
use opentelemetry_otlp::{WithExportConfig, WithTonicConfig};
use opentelemetry_resource_detectors::{
    HostResourceDetector, OsResourceDetector, ProcessResourceDetector,
};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use tonic::metadata::MetadataMap;

pub fn init_logger_provider(
    dsn: String,
) -> Result<SdkLoggerProvider, Box<dyn std::error::Error + Send + Sync + 'static>>
{
    //gRPC metadata
    let mut metadata = MetadataMap::with_capacity(1);
    metadata.insert("uptrace-dsn", dsn.parse().unwrap());

    let exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .with_tls_config(
            tonic::transport::ClientTlsConfig::new().with_native_roots(),
        )
        .with_endpoint("https://api.uptrace.dev:4317")
        .with_metadata(metadata)
        .build()?;

    let provider = SdkLoggerProvider::builder()
        .with_resource(build_resource())
        .with_batch_exporter(exporter)
        .build();

    Ok(provider)
}

fn build_resource() -> Resource {
    Resource::builder()
        .with_detector(Box::new(OsResourceDetector))
        .with_detector(Box::new(HostResourceDetector::default()))
        .with_detector(Box::new(ProcessResourceDetector))
        .with_attributes([
            KeyValue::new("service.name", "ytfilter"),
            KeyValue::new("service.version", "0.1.0"),
            KeyValue::new("deployment.environment", "production"),
        ])
        .build()
}
