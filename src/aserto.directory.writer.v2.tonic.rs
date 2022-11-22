// @generated
/// Generated client implementations.
pub mod writer_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct WriterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WriterClient<tonic::transport::Channel> {
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
    impl<T> WriterClient<T>
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
        ) -> WriterClient<InterceptedService<T, F>>
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
            WriterClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn set_object_type(
            &mut self,
            request: impl tonic::IntoRequest<super::SetObjectTypeRequest>,
        ) -> Result<tonic::Response<super::SetObjectTypeResponse>, tonic::Status> {
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
                "/aserto.directory.writer.v2.Writer/SetObjectType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_object_type(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectTypeRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectTypeResponse>, tonic::Status> {
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
                "/aserto.directory.writer.v2.Writer/DeleteObjectType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_relation_type(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRelationTypeRequest>,
        ) -> Result<tonic::Response<super::SetRelationTypeResponse>, tonic::Status> {
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
                "/aserto.directory.writer.v2.Writer/SetRelationType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_relation_type(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRelationTypeRequest>,
        ) -> Result<tonic::Response<super::DeleteRelationTypeResponse>, tonic::Status> {
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
                "/aserto.directory.writer.v2.Writer/DeleteRelationType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPermissionRequest>,
        ) -> Result<tonic::Response<super::SetPermissionResponse>, tonic::Status> {
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
                "/aserto.directory.writer.v2.Writer/SetPermission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePermissionRequest>,
        ) -> Result<tonic::Response<super::DeletePermissionResponse>, tonic::Status> {
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
                "/aserto.directory.writer.v2.Writer/DeletePermission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_object(
            &mut self,
            request: impl tonic::IntoRequest<super::SetObjectRequest>,
        ) -> Result<tonic::Response<super::SetObjectResponse>, tonic::Status> {
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
                "/aserto.directory.writer.v2.Writer/SetObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_object(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectResponse>, tonic::Status> {
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
                "/aserto.directory.writer.v2.Writer/DeleteObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_relation(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRelationRequest>,
        ) -> Result<tonic::Response<super::SetRelationResponse>, tonic::Status> {
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
                "/aserto.directory.writer.v2.Writer/SetRelation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_relation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRelationRequest>,
        ) -> Result<tonic::Response<super::DeleteRelationResponse>, tonic::Status> {
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
                "/aserto.directory.writer.v2.Writer/DeleteRelation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod writer_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with WriterServer.
    #[async_trait]
    pub trait Writer: Send + Sync + 'static {
        async fn set_object_type(
            &self,
            request: tonic::Request<super::SetObjectTypeRequest>,
        ) -> Result<tonic::Response<super::SetObjectTypeResponse>, tonic::Status>;
        async fn delete_object_type(
            &self,
            request: tonic::Request<super::DeleteObjectTypeRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectTypeResponse>, tonic::Status>;
        async fn set_relation_type(
            &self,
            request: tonic::Request<super::SetRelationTypeRequest>,
        ) -> Result<tonic::Response<super::SetRelationTypeResponse>, tonic::Status>;
        async fn delete_relation_type(
            &self,
            request: tonic::Request<super::DeleteRelationTypeRequest>,
        ) -> Result<tonic::Response<super::DeleteRelationTypeResponse>, tonic::Status>;
        async fn set_permission(
            &self,
            request: tonic::Request<super::SetPermissionRequest>,
        ) -> Result<tonic::Response<super::SetPermissionResponse>, tonic::Status>;
        async fn delete_permission(
            &self,
            request: tonic::Request<super::DeletePermissionRequest>,
        ) -> Result<tonic::Response<super::DeletePermissionResponse>, tonic::Status>;
        async fn set_object(
            &self,
            request: tonic::Request<super::SetObjectRequest>,
        ) -> Result<tonic::Response<super::SetObjectResponse>, tonic::Status>;
        async fn delete_object(
            &self,
            request: tonic::Request<super::DeleteObjectRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectResponse>, tonic::Status>;
        async fn set_relation(
            &self,
            request: tonic::Request<super::SetRelationRequest>,
        ) -> Result<tonic::Response<super::SetRelationResponse>, tonic::Status>;
        async fn delete_relation(
            &self,
            request: tonic::Request<super::DeleteRelationRequest>,
        ) -> Result<tonic::Response<super::DeleteRelationResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct WriterServer<T: Writer> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Writer> WriterServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WriterServer<T>
    where
        T: Writer,
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
                "/aserto.directory.writer.v2.Writer/SetObjectType" => {
                    #[allow(non_camel_case_types)]
                    struct SetObjectTypeSvc<T: Writer>(pub Arc<T>);
                    impl<
                        T: Writer,
                    > tonic::server::UnaryService<super::SetObjectTypeRequest>
                    for SetObjectTypeSvc<T> {
                        type Response = super::SetObjectTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetObjectTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_object_type(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetObjectTypeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aserto.directory.writer.v2.Writer/DeleteObjectType" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteObjectTypeSvc<T: Writer>(pub Arc<T>);
                    impl<
                        T: Writer,
                    > tonic::server::UnaryService<super::DeleteObjectTypeRequest>
                    for DeleteObjectTypeSvc<T> {
                        type Response = super::DeleteObjectTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteObjectTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_object_type(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteObjectTypeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aserto.directory.writer.v2.Writer/SetRelationType" => {
                    #[allow(non_camel_case_types)]
                    struct SetRelationTypeSvc<T: Writer>(pub Arc<T>);
                    impl<
                        T: Writer,
                    > tonic::server::UnaryService<super::SetRelationTypeRequest>
                    for SetRelationTypeSvc<T> {
                        type Response = super::SetRelationTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRelationTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_relation_type(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRelationTypeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aserto.directory.writer.v2.Writer/DeleteRelationType" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRelationTypeSvc<T: Writer>(pub Arc<T>);
                    impl<
                        T: Writer,
                    > tonic::server::UnaryService<super::DeleteRelationTypeRequest>
                    for DeleteRelationTypeSvc<T> {
                        type Response = super::DeleteRelationTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRelationTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_relation_type(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteRelationTypeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aserto.directory.writer.v2.Writer/SetPermission" => {
                    #[allow(non_camel_case_types)]
                    struct SetPermissionSvc<T: Writer>(pub Arc<T>);
                    impl<
                        T: Writer,
                    > tonic::server::UnaryService<super::SetPermissionRequest>
                    for SetPermissionSvc<T> {
                        type Response = super::SetPermissionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetPermissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_permission(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetPermissionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aserto.directory.writer.v2.Writer/DeletePermission" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePermissionSvc<T: Writer>(pub Arc<T>);
                    impl<
                        T: Writer,
                    > tonic::server::UnaryService<super::DeletePermissionRequest>
                    for DeletePermissionSvc<T> {
                        type Response = super::DeletePermissionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePermissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_permission(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeletePermissionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aserto.directory.writer.v2.Writer/SetObject" => {
                    #[allow(non_camel_case_types)]
                    struct SetObjectSvc<T: Writer>(pub Arc<T>);
                    impl<T: Writer> tonic::server::UnaryService<super::SetObjectRequest>
                    for SetObjectSvc<T> {
                        type Response = super::SetObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aserto.directory.writer.v2.Writer/DeleteObject" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteObjectSvc<T: Writer>(pub Arc<T>);
                    impl<
                        T: Writer,
                    > tonic::server::UnaryService<super::DeleteObjectRequest>
                    for DeleteObjectSvc<T> {
                        type Response = super::DeleteObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aserto.directory.writer.v2.Writer/SetRelation" => {
                    #[allow(non_camel_case_types)]
                    struct SetRelationSvc<T: Writer>(pub Arc<T>);
                    impl<
                        T: Writer,
                    > tonic::server::UnaryService<super::SetRelationRequest>
                    for SetRelationSvc<T> {
                        type Response = super::SetRelationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRelationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_relation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRelationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aserto.directory.writer.v2.Writer/DeleteRelation" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRelationSvc<T: Writer>(pub Arc<T>);
                    impl<
                        T: Writer,
                    > tonic::server::UnaryService<super::DeleteRelationRequest>
                    for DeleteRelationSvc<T> {
                        type Response = super::DeleteRelationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRelationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_relation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteRelationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
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
    impl<T: Writer> Clone for WriterServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Writer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Writer> tonic::server::NamedService for WriterServer<T> {
        const NAME: &'static str = "aserto.directory.writer.v2.Writer";
    }
}
