/// EchoRequest is the request for echo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoRequest {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
}
/// EchoResponse is the response for echo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoResponse {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
}
#[doc = r" Generated server implementations."]
pub mod echo_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with EchoServer."]
    #[async_trait]
    pub trait Echo: Send + Sync + 'static {
        #[doc = " UnaryEcho is unary echo."]
        async fn unary_echo(
            &self,
            request: tonic::Request<super::EchoRequest>,
        ) -> Result<tonic::Response<super::EchoResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the ServerStreamingEcho method."]
        type ServerStreamingEchoStream: Stream<Item = Result<super::EchoResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " ServerStreamingEcho is server side streaming."]
        async fn server_streaming_echo(
            &self,
            request: tonic::Request<super::EchoRequest>,
        ) -> Result<tonic::Response<Self::ServerStreamingEchoStream>, tonic::Status>;
        #[doc = " ClientStreamingEcho is client side streaming."]
        async fn client_streaming_echo(
            &self,
            request: tonic::Request<tonic::Streaming<super::EchoRequest>>,
        ) -> Result<tonic::Response<super::EchoResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the BidirectionalStreamingEcho method."]
        type BidirectionalStreamingEchoStream: Stream<Item = Result<super::EchoResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " BidirectionalStreamingEcho is bidi streaming."]
        async fn bidirectional_streaming_echo(
            &self,
            request: tonic::Request<tonic::Streaming<super::EchoRequest>>,
        ) -> Result<tonic::Response<Self::BidirectionalStreamingEchoStream>, tonic::Status>;
    }
    #[doc = " Echo is the echo service."]
    #[derive(Debug)]
    pub struct EchoServer<T: Echo> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Echo> EchoServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for EchoServer<T>
    where
        T: Echo,
        B: HttpBody + Send + Sync + 'static,
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
                "/echo.Echo/UnaryEcho" => {
                    #[allow(non_camel_case_types)]
                    struct UnaryEchoSvc<T: Echo>(pub Arc<T>);
                    impl<T: Echo> tonic::server::UnaryService<super::EchoRequest> for UnaryEchoSvc<T> {
                        type Response = super::EchoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EchoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).unary_echo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UnaryEchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/echo.Echo/ServerStreamingEcho" => {
                    #[allow(non_camel_case_types)]
                    struct ServerStreamingEchoSvc<T: Echo>(pub Arc<T>);
                    impl<T: Echo> tonic::server::ServerStreamingService<super::EchoRequest>
                        for ServerStreamingEchoSvc<T>
                    {
                        type Response = super::EchoResponse;
                        type ResponseStream = T::ServerStreamingEchoStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EchoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).server_streaming_echo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ServerStreamingEchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/echo.Echo/ClientStreamingEcho" => {
                    #[allow(non_camel_case_types)]
                    struct ClientStreamingEchoSvc<T: Echo>(pub Arc<T>);
                    impl<T: Echo> tonic::server::ClientStreamingService<super::EchoRequest>
                        for ClientStreamingEchoSvc<T>
                    {
                        type Response = super::EchoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::EchoRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).client_streaming_echo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ClientStreamingEchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/echo.Echo/BidirectionalStreamingEcho" => {
                    #[allow(non_camel_case_types)]
                    struct BidirectionalStreamingEchoSvc<T: Echo>(pub Arc<T>);
                    impl<T: Echo> tonic::server::StreamingService<super::EchoRequest>
                        for BidirectionalStreamingEchoSvc<T>
                    {
                        type Response = super::EchoResponse;
                        type ResponseStream = T::BidirectionalStreamingEchoStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::EchoRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).bidirectional_streaming_echo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = BidirectionalStreamingEchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Echo> Clone for EchoServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Echo> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Echo> tonic::transport::NamedService for EchoServer<T> {
        const NAME: &'static str = "echo.Echo";
    }
}
