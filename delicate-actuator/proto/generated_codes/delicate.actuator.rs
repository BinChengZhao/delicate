#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    #[prost(uint64, tag = "1")]
    pub task_id: u64,
    #[prost(string, tag = "2")]
    pub task_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub command: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnifiedResponseMessages {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub data: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
#[doc = r" Generated client implementations."]
pub mod actuator_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ActuatorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ActuatorClient<tonic::transport::Channel> {
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
    impl<T> ActuatorClient<T>
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
        ) -> ActuatorClient<InterceptedService<T, F>>
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
            ActuatorClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn add_task(
            &mut self,
            request: impl tonic::IntoRequest<super::Task>,
        ) -> Result<tonic::Response<super::UnifiedResponseMessages>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/delicate.actuator.Actuator/AddTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod actuator_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ActuatorServer."]
    #[async_trait]
    pub trait Actuator: Send + Sync + 'static {
        async fn add_task(
            &self,
            request: tonic::Request<super::Task>,
        ) -> Result<tonic::Response<super::UnifiedResponseMessages>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ActuatorServer<T: Actuator> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Actuator> ActuatorServer<T> {
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ActuatorServer<T>
    where
        T: Actuator,
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
                "/delicate.actuator.Actuator/AddTask" => {
                    #[allow(non_camel_case_types)]
                    struct AddTaskSvc<T: Actuator>(pub Arc<T>);
                    impl<T: Actuator> tonic::server::UnaryService<super::Task> for AddTaskSvc<T> {
                        type Response = super::UnifiedResponseMessages;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Task>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
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
    impl<T: Actuator> Clone for ActuatorServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Actuator> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Actuator> tonic::transport::NamedService for ActuatorServer<T> {
        const NAME: &'static str = "delicate.actuator.Actuator";
    }
}
