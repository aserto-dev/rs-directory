// @generated
/// Generated client implementations.
pub mod reader_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ReaderClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReaderClient<tonic::transport::Channel> {
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
    impl<T> ReaderClient<T>
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
        ) -> ReaderClient<InterceptedService<T, F>>
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
            ReaderClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_object_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectTypeRequest>,
        ) -> Result<tonic::Response<super::GetObjectTypeResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/GetObjectType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_object_types(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectTypesRequest>,
        ) -> Result<tonic::Response<super::GetObjectTypesResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/GetObjectTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_relation_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRelationTypeRequest>,
        ) -> Result<tonic::Response<super::GetRelationTypeResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/GetRelationType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_relation_types(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRelationTypesRequest>,
        ) -> Result<tonic::Response<super::GetRelationTypesResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/GetRelationTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPermissionRequest>,
        ) -> Result<tonic::Response<super::GetPermissionResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/GetPermission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPermissionsRequest>,
        ) -> Result<tonic::Response<super::GetPermissionsResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/GetPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_object(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectRequest>,
        ) -> Result<tonic::Response<super::GetObjectResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/GetObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_object_many(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectManyRequest>,
        ) -> Result<tonic::Response<super::GetObjectManyResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/GetObjectMany",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectsRequest>,
        ) -> Result<tonic::Response<super::GetObjectsResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/GetObjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_relation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRelationRequest>,
        ) -> Result<tonic::Response<super::GetRelationResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/GetRelation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_relations(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRelationsRequest>,
        ) -> Result<tonic::Response<super::GetRelationsResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/GetRelations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn check_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckPermissionRequest>,
        ) -> Result<tonic::Response<super::CheckPermissionResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/CheckPermission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn check_relation(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckRelationRequest>,
        ) -> Result<tonic::Response<super::CheckRelationResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/CheckRelation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_graph(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGraphRequest>,
        ) -> Result<tonic::Response<super::GetGraphResponse>, tonic::Status> {
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
                "/aserto.directory.reader.v2.Reader/GetGraph",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod reader_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ReaderServer.
    #[async_trait]
    pub trait Reader: Send + Sync + 'static {
        async fn get_object_type(
            &self,
            request: tonic::Request<super::GetObjectTypeRequest>,
        ) -> Result<tonic::Response<super::GetObjectTypeResponse>, tonic::Status>;
        async fn get_object_types(
            &self,
            request: tonic::Request<super::GetObjectTypesRequest>,
        ) -> Result<tonic::Response<super::GetObjectTypesResponse>, tonic::Status>;
        async fn get_relation_type(
            &self,
            request: tonic::Request<super::GetRelationTypeRequest>,
        ) -> Result<tonic::Response<super::GetRelationTypeResponse>, tonic::Status>;
        async fn get_relation_types(
            &self,
            request: tonic::Request<super::GetRelationTypesRequest>,
        ) -> Result<tonic::Response<super::GetRelationTypesResponse>, tonic::Status>;
        async fn get_permission(
            &self,
            request: tonic::Request<super::GetPermissionRequest>,
        ) -> Result<tonic::Response<super::GetPermissionResponse>, tonic::Status>;
        async fn get_permissions(
            &self,
            request: tonic::Request<super::GetPermissionsRequest>,
        ) -> Result<tonic::Response<super::GetPermissionsResponse>, tonic::Status>;
        async fn get_object(
            &self,
            request: tonic::Request<super::GetObjectRequest>,
        ) -> Result<tonic::Response<super::GetObjectResponse>, tonic::Status>;
        async fn get_object_many(
            &self,
            request: tonic::Request<super::GetObjectManyRequest>,
        ) -> Result<tonic::Response<super::GetObjectManyResponse>, tonic::Status>;
        async fn get_objects(
            &self,
            request: tonic::Request<super::GetObjectsRequest>,
        ) -> Result<tonic::Response<super::GetObjectsResponse>, tonic::Status>;
        async fn get_relation(
            &self,
            request: tonic::Request<super::GetRelationRequest>,
        ) -> Result<tonic::Response<super::GetRelationResponse>, tonic::Status>;
        async fn get_relations(
            &self,
            request: tonic::Request<super::GetRelationsRequest>,
        ) -> Result<tonic::Response<super::GetRelationsResponse>, tonic::Status>;
        async fn check_permission(
            &self,
            request: tonic::Request<super::CheckPermissionRequest>,
        ) -> Result<tonic::Response<super::CheckPermissionResponse>, tonic::Status>;
        async fn check_relation(
            &self,
            request: tonic::Request<super::CheckRelationRequest>,
        ) -> Result<tonic::Response<super::CheckRelationResponse>, tonic::Status>;
        async fn get_graph(
            &self,
            request: tonic::Request<super::GetGraphRequest>,
        ) -> Result<tonic::Response<super::GetGraphResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ReaderServer<T: Reader> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Reader> ReaderServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ReaderServer<T>
    where
        T: Reader,
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
                "/aserto.directory.reader.v2.Reader/GetObjectType" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectTypeSvc<T: Reader>(pub Arc<T>);
                    impl<
                        T: Reader,
                    > tonic::server::UnaryService<super::GetObjectTypeRequest>
                    for GetObjectTypeSvc<T> {
                        type Response = super::GetObjectTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_type(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectTypeSvc(inner);
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
                "/aserto.directory.reader.v2.Reader/GetObjectTypes" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectTypesSvc<T: Reader>(pub Arc<T>);
                    impl<
                        T: Reader,
                    > tonic::server::UnaryService<super::GetObjectTypesRequest>
                    for GetObjectTypesSvc<T> {
                        type Response = super::GetObjectTypesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectTypesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_types(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectTypesSvc(inner);
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
                "/aserto.directory.reader.v2.Reader/GetRelationType" => {
                    #[allow(non_camel_case_types)]
                    struct GetRelationTypeSvc<T: Reader>(pub Arc<T>);
                    impl<
                        T: Reader,
                    > tonic::server::UnaryService<super::GetRelationTypeRequest>
                    for GetRelationTypeSvc<T> {
                        type Response = super::GetRelationTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRelationTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_relation_type(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRelationTypeSvc(inner);
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
                "/aserto.directory.reader.v2.Reader/GetRelationTypes" => {
                    #[allow(non_camel_case_types)]
                    struct GetRelationTypesSvc<T: Reader>(pub Arc<T>);
                    impl<
                        T: Reader,
                    > tonic::server::UnaryService<super::GetRelationTypesRequest>
                    for GetRelationTypesSvc<T> {
                        type Response = super::GetRelationTypesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRelationTypesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_relation_types(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRelationTypesSvc(inner);
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
                "/aserto.directory.reader.v2.Reader/GetPermission" => {
                    #[allow(non_camel_case_types)]
                    struct GetPermissionSvc<T: Reader>(pub Arc<T>);
                    impl<
                        T: Reader,
                    > tonic::server::UnaryService<super::GetPermissionRequest>
                    for GetPermissionSvc<T> {
                        type Response = super::GetPermissionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPermissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_permission(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPermissionSvc(inner);
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
                "/aserto.directory.reader.v2.Reader/GetPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct GetPermissionsSvc<T: Reader>(pub Arc<T>);
                    impl<
                        T: Reader,
                    > tonic::server::UnaryService<super::GetPermissionsRequest>
                    for GetPermissionsSvc<T> {
                        type Response = super::GetPermissionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPermissionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_permissions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPermissionsSvc(inner);
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
                "/aserto.directory.reader.v2.Reader/GetObject" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectSvc<T: Reader>(pub Arc<T>);
                    impl<T: Reader> tonic::server::UnaryService<super::GetObjectRequest>
                    for GetObjectSvc<T> {
                        type Response = super::GetObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectSvc(inner);
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
                "/aserto.directory.reader.v2.Reader/GetObjectMany" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectManySvc<T: Reader>(pub Arc<T>);
                    impl<
                        T: Reader,
                    > tonic::server::UnaryService<super::GetObjectManyRequest>
                    for GetObjectManySvc<T> {
                        type Response = super::GetObjectManyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectManyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_many(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectManySvc(inner);
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
                "/aserto.directory.reader.v2.Reader/GetObjects" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectsSvc<T: Reader>(pub Arc<T>);
                    impl<T: Reader> tonic::server::UnaryService<super::GetObjectsRequest>
                    for GetObjectsSvc<T> {
                        type Response = super::GetObjectsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_objects(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectsSvc(inner);
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
                "/aserto.directory.reader.v2.Reader/GetRelation" => {
                    #[allow(non_camel_case_types)]
                    struct GetRelationSvc<T: Reader>(pub Arc<T>);
                    impl<
                        T: Reader,
                    > tonic::server::UnaryService<super::GetRelationRequest>
                    for GetRelationSvc<T> {
                        type Response = super::GetRelationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRelationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_relation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRelationSvc(inner);
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
                "/aserto.directory.reader.v2.Reader/GetRelations" => {
                    #[allow(non_camel_case_types)]
                    struct GetRelationsSvc<T: Reader>(pub Arc<T>);
                    impl<
                        T: Reader,
                    > tonic::server::UnaryService<super::GetRelationsRequest>
                    for GetRelationsSvc<T> {
                        type Response = super::GetRelationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRelationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_relations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRelationsSvc(inner);
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
                "/aserto.directory.reader.v2.Reader/CheckPermission" => {
                    #[allow(non_camel_case_types)]
                    struct CheckPermissionSvc<T: Reader>(pub Arc<T>);
                    impl<
                        T: Reader,
                    > tonic::server::UnaryService<super::CheckPermissionRequest>
                    for CheckPermissionSvc<T> {
                        type Response = super::CheckPermissionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckPermissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).check_permission(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckPermissionSvc(inner);
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
                "/aserto.directory.reader.v2.Reader/CheckRelation" => {
                    #[allow(non_camel_case_types)]
                    struct CheckRelationSvc<T: Reader>(pub Arc<T>);
                    impl<
                        T: Reader,
                    > tonic::server::UnaryService<super::CheckRelationRequest>
                    for CheckRelationSvc<T> {
                        type Response = super::CheckRelationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckRelationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).check_relation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckRelationSvc(inner);
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
                "/aserto.directory.reader.v2.Reader/GetGraph" => {
                    #[allow(non_camel_case_types)]
                    struct GetGraphSvc<T: Reader>(pub Arc<T>);
                    impl<T: Reader> tonic::server::UnaryService<super::GetGraphRequest>
                    for GetGraphSvc<T> {
                        type Response = super::GetGraphResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGraphRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_graph(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetGraphSvc(inner);
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
    impl<T: Reader> Clone for ReaderServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Reader> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Reader> tonic::server::NamedService for ReaderServer<T> {
        const NAME: &'static str = "aserto.directory.reader.v2.Reader";
    }
}
