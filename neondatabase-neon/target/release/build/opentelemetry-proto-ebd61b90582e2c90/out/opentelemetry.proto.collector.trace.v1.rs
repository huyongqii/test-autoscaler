#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTraceServiceRequest {
    /// An array of ResourceSpans.
    /// For data coming from a single resource this array will typically contain one
    /// element. Intermediary nodes (such as OpenTelemetry Collector) that receive
    /// data from multiple origins typically batch the data before forwarding further and
    /// in that case this array will contain multiple elements.
    #[prost(message, repeated, tag = "1")]
    pub resource_spans: ::prost::alloc::vec::Vec<
        super::super::super::trace::v1::ResourceSpans,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTraceServiceResponse {}
