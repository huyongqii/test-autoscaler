#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeSafekeeperInfoRequest {
    #[prost(oneof = "subscribe_safekeeper_info_request::SubscriptionKey", tags = "1, 2")]
    pub subscription_key: ::core::option::Option<
        subscribe_safekeeper_info_request::SubscriptionKey,
    >,
}
/// Nested message and enum types in `SubscribeSafekeeperInfoRequest`.
pub mod subscribe_safekeeper_info_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SubscriptionKey {
        /// subscribe to everything
        #[prost(message, tag = "1")]
        All(()),
        /// subscribe to specific timeline
        #[prost(message, tag = "2")]
        TenantTimelineId(super::TenantTimelineId),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafekeeperTimelineInfo {
    #[prost(uint64, tag = "1")]
    pub safekeeper_id: u64,
    #[prost(message, optional, tag = "2")]
    pub tenant_timeline_id: ::core::option::Option<TenantTimelineId>,
    /// Term of the last entry.
    #[prost(uint64, tag = "3")]
    pub last_log_term: u64,
    /// LSN of the last record.
    #[prost(uint64, tag = "4")]
    pub flush_lsn: u64,
    /// Up to which LSN safekeeper regards its WAL as committed.
    #[prost(uint64, tag = "5")]
    pub commit_lsn: u64,
    /// LSN up to which safekeeper has backed WAL.
    #[prost(uint64, tag = "6")]
    pub backup_lsn: u64,
    /// LSN of last checkpoint uploaded by pageserver.
    #[prost(uint64, tag = "7")]
    pub remote_consistent_lsn: u64,
    #[prost(uint64, tag = "8")]
    pub peer_horizon_lsn: u64,
    #[prost(uint64, tag = "9")]
    pub local_start_lsn: u64,
    /// A connection string to use for WAL receiving.
    #[prost(string, tag = "10")]
    pub safekeeper_connstr: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TenantTimelineId {
    #[prost(bytes = "vec", tag = "1")]
    pub tenant_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub timeline_id: ::prost::alloc::vec::Vec<u8>,
}
/// Generated client implementations.
pub mod broker_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BrokerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BrokerServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BrokerServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BrokerServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            BrokerServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Subscribe to safekeeper updates.
        pub async fn subscribe_safekeeper_info(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeSafekeeperInfoRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::SafekeeperTimelineInfo>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/storage_broker.BrokerService/SubscribeSafekeeperInfo",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Publish safekeeper updates.
        pub async fn publish_safekeeper_info(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::SafekeeperTimelineInfo,
            >,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/storage_broker.BrokerService/PublishSafekeeperInfo",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
/// Generated server implementations.
pub mod broker_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BrokerServiceServer.
    #[async_trait]
    pub trait BrokerService: Send + Sync + 'static {
        /// Server streaming response type for the SubscribeSafekeeperInfo method.
        type SubscribeSafekeeperInfoStream: futures_core::Stream<
                Item = Result<super::SafekeeperTimelineInfo, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to safekeeper updates.
        async fn subscribe_safekeeper_info(
            &self,
            request: tonic::Request<super::SubscribeSafekeeperInfoRequest>,
        ) -> Result<tonic::Response<Self::SubscribeSafekeeperInfoStream>, tonic::Status>;
        /// Publish safekeeper updates.
        async fn publish_safekeeper_info(
            &self,
            request: tonic::Request<tonic::Streaming<super::SafekeeperTimelineInfo>>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BrokerServiceServer<T: BrokerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BrokerService> BrokerServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BrokerServiceServer<T>
    where
        T: BrokerService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/storage_broker.BrokerService/SubscribeSafekeeperInfo" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeSafekeeperInfoSvc<T: BrokerService>(pub Arc<T>);
                    impl<
                        T: BrokerService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeSafekeeperInfoRequest,
                    > for SubscribeSafekeeperInfoSvc<T> {
                        type Response = super::SafekeeperTimelineInfo;
                        type ResponseStream = T::SubscribeSafekeeperInfoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeSafekeeperInfoRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_safekeeper_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeSafekeeperInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/storage_broker.BrokerService/PublishSafekeeperInfo" => {
                    #[allow(non_camel_case_types)]
                    struct PublishSafekeeperInfoSvc<T: BrokerService>(pub Arc<T>);
                    impl<
                        T: BrokerService,
                    > tonic::server::ClientStreamingService<
                        super::SafekeeperTimelineInfo,
                    > for PublishSafekeeperInfoSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SafekeeperTimelineInfo>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).publish_safekeeper_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PublishSafekeeperInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: BrokerService> Clone for BrokerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: BrokerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BrokerService> tonic::server::NamedService for BrokerServiceServer<T> {
        const NAME: &'static str = "storage_broker.BrokerService";
    }
}
