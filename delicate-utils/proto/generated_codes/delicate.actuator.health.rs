#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckRequest {
    #[prost(int64, tag = "6")]
    pub time: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemSnapshot {
    #[prost(message, optional, tag = "1")]
    pub processor: ::core::option::Option<system_snapshot::Processor>,
    #[prost(message, optional, tag = "2")]
    pub memory: ::core::option::Option<system_snapshot::Memory>,
}
/// Nested message and enum types in `SystemSnapshot`.
pub mod system_snapshot {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Processor {
        #[prost(float, tag = "1")]
        pub cpu_usage: f32,
        #[prost(uint64, tag = "2")]
        pub frequency: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Memory {
        #[prost(uint64, tag = "1")]
        pub total_memory: u64,
        #[prost(uint64, tag = "2")]
        pub used_memory: u64,
        #[prost(uint64, tag = "3")]
        pub free_memory: u64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckResponse {
    #[prost(enumeration = "health_check_response::ServingStatus", tag = "1")]
    pub status: i32,
    #[prost(message, optional, tag = "2")]
    pub system_snapshot: ::core::option::Option<SystemSnapshot>,
    #[prost(message, optional, tag = "3")]
    pub bind_request: ::core::option::Option<super::BindRequest>,
}
/// Nested message and enum types in `HealthCheckResponse`.
pub mod health_check_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ServingStatus {
        Unknown = 0,
        Serving = 1,
        NotServing = 2,
        /// `SERVICE_UNKNOWN` Used only by the Watch method.
        ServiceUnknown = 3,
    }
}
#[doc = r" Generated client implementations."]
pub mod health_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct HealthClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl HealthClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> HealthClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> HealthClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            HealthClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " If the requested service is unknown, the call will fail with status"]
        #[doc = " NOT_FOUND."]
        pub async fn check(
            &mut self,
            request: impl tonic::IntoRequest<super::HealthCheckRequest>,
        ) -> Result<tonic::Response<super::super::UnifiedResponseMessagesForGrpc>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/delicate.actuator.health.Health/Check");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Performs a watch for the serving status of the requested service."]
        #[doc = " The server will immediately send back a message indicating the current"]
        #[doc = " serving status.  It will then subsequently send a new message whenever"]
        #[doc = " the service's serving status changes."]
        #[doc = ""]
        #[doc = " If the requested service is unknown when the call is received, the"]
        #[doc = " server will send a message setting the serving status to"]
        #[doc = " SERVICE_UNKNOWN but will *not* terminate the call.  If at some"]
        #[doc = " future point, the serving status of the service becomes known, the"]
        #[doc = " server will send a new message with the service's serving status."]
        #[doc = ""]
        #[doc = " If the call terminates with status UNIMPLEMENTED, then clients"]
        #[doc = " should assume this method is not supported and should not retry the"]
        #[doc = " call.  If the call terminates with any other status (including OK),"]
        #[doc = " clients should retry the call with appropriate exponential backoff."]
        pub async fn watch(
            &mut self,
            request: impl tonic::IntoRequest<super::HealthCheckRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::UnifiedResponseMessagesForGrpc>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/delicate.actuator.health.Health/Watch");
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod health_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with HealthServer."]
    #[async_trait]
    pub trait Health: Send + Sync + 'static {
        #[doc = " If the requested service is unknown, the call will fail with status"]
        #[doc = " NOT_FOUND."]
        async fn check(
            &self,
            request: tonic::Request<super::HealthCheckRequest>,
        ) -> Result<tonic::Response<super::super::UnifiedResponseMessagesForGrpc>, tonic::Status>;
        #[doc = "Server streaming response type for the Watch method."]
        type WatchStream: futures_core::Stream<
                Item = Result<super::super::UnifiedResponseMessagesForGrpc, tonic::Status>,
            > + Send
            + Sync
            + 'static;
        #[doc = " Performs a watch for the serving status of the requested service."]
        #[doc = " The server will immediately send back a message indicating the current"]
        #[doc = " serving status.  It will then subsequently send a new message whenever"]
        #[doc = " the service's serving status changes."]
        #[doc = ""]
        #[doc = " If the requested service is unknown when the call is received, the"]
        #[doc = " server will send a message setting the serving status to"]
        #[doc = " SERVICE_UNKNOWN but will *not* terminate the call.  If at some"]
        #[doc = " future point, the serving status of the service becomes known, the"]
        #[doc = " server will send a new message with the service's serving status."]
        #[doc = ""]
        #[doc = " If the call terminates with status UNIMPLEMENTED, then clients"]
        #[doc = " should assume this method is not supported and should not retry the"]
        #[doc = " call.  If the call terminates with any other status (including OK),"]
        #[doc = " clients should retry the call with appropriate exponential backoff."]
        async fn watch(
            &self,
            request: tonic::Request<super::HealthCheckRequest>,
        ) -> Result<tonic::Response<Self::WatchStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct HealthServer<T: Health> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Health> HealthServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        #[doc = r" Enable decompressing requests with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        #[doc = r" Compress responses with `gzip`, if the client supports it."]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for HealthServer<T>
    where
        T: Health,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/delicate.actuator.health.Health/Check" => {
                    #[allow(non_camel_case_types)]
                    struct CheckSvc<T: Health>(pub Arc<T>);
                    impl<T: Health> tonic::server::UnaryService<super::HealthCheckRequest> for CheckSvc<T> {
                        type Response = super::super::UnifiedResponseMessagesForGrpc;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HealthCheckRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).check(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                "/delicate.actuator.health.Health/Watch" => {
                    #[allow(non_camel_case_types)]
                    struct WatchSvc<T: Health>(pub Arc<T>);
                    impl<T: Health> tonic::server::ServerStreamingService<super::HealthCheckRequest> for WatchSvc<T> {
                        type Response = super::super::UnifiedResponseMessagesForGrpc;
                        type ResponseStream = T::WatchStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HealthCheckRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).watch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WatchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                },
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Health> Clone for HealthServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Health> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Health> tonic::transport::NamedService for HealthServer<T> {
        const NAME: &'static str = "delicate.actuator.health.Health";
    }
}
