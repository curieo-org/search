use async_trait::async_trait;
use oauth2::http::Uri;
use server::proto::agency_service_client::AgencyServiceClient;
use server::proto::agency_service_server::{AgencyService, AgencyServiceServer};
use server::proto::{Embeddings, EmbeddingsOutput, PubmedResponse, PubmedSource, SearchInput};
use std::future::Future;
use std::sync::Arc;
use tempfile::NamedTempFile;
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::{Channel, Endpoint, Server};
use tonic::{Request, Response, Status};
use tower::service_fn;

struct MockAgencyServer {
    pubmed_parent_search_response: PubmedResponse,
    pubmed_cluster_search_response: PubmedResponse,
    embeddings_compute_response: EmbeddingsOutput,
}

impl MockAgencyServer {
    fn set_pubmed_parent_search_response(&mut self, response: PubmedResponse) {
        self.pubmed_parent_search_response = response
    }
    fn set_pubmed_cluster_search_response(&mut self, response: PubmedResponse) {
        self.pubmed_cluster_search_response = response
    }
    fn set_embeddings_compute_response(&mut self, response: EmbeddingsOutput) {
        self.embeddings_compute_response = response
    }
}

#[async_trait]
impl AgencyService for MockAgencyServer {
    async fn pubmed_parent_search(
        &self,
        _: Request<Embeddings>,
    ) -> std::result::Result<Response<PubmedResponse>, Status> {
        Ok(Response::new(self.pubmed_parent_search_response.clone()))
    }

    async fn pubmed_cluster_search(
        &self,
        _: Request<Embeddings>,
    ) -> std::result::Result<Response<PubmedResponse>, Status> {
        Ok(Response::new(self.pubmed_cluster_search_response.clone()))
    }

    async fn embeddings_compute(
        &self,
        _: Request<SearchInput>,
    ) -> std::result::Result<Response<EmbeddingsOutput>, Status> {
        Ok(Response::new(self.embeddings_compute_response.clone()))
    }
}

pub async fn agency_server_and_client_stub() -> (
    impl Future<Output = ()> + Sized,
    AgencyServiceClient<Channel>,
) {
    let socket = NamedTempFile::new().unwrap();
    let socket = Arc::new(socket.into_temp_path());
    std::fs::remove_file(&*socket).unwrap();

    let uds = UnixListener::bind(&*socket).unwrap();
    let stream = UnixListenerStream::new(uds);

    let mock_server = MockAgencyServer {
        pubmed_parent_search_response: PubmedResponse {
            status: 200,
            sources: vec![PubmedSource {
                pubmed_id: "test-pubmed-id".to_string(),
                title: "test-title".to_string(),
                r#abstract: "test-abstract".to_string(),
                embeddings: Some(Embeddings {
                    dense_embedding: vec![],
                    sparse_embedding: vec![],
                    sparse_indices: vec![],
                }),
            }],
        },
        pubmed_cluster_search_response: PubmedResponse {
            status: 200,
            sources: vec![PubmedSource {
                pubmed_id: "test-pubmed-id".to_string(),
                title: "test-title".to_string(),
                r#abstract: "test-abstract".to_string(),
                embeddings: Some(Embeddings {
                    dense_embedding: vec![],
                    sparse_embedding: vec![],
                    sparse_indices: vec![],
                }),
            }],
        },
        embeddings_compute_response: EmbeddingsOutput {
            status: 200,
            embeddings: Some(Embeddings {
                dense_embedding: vec![],
                sparse_embedding: vec![],
                sparse_indices: vec![],
            }),
        },
    };

    let agency_service_server = AgencyServiceServer::new(mock_server);
    let server = async {
        let server = Server::builder()
            .add_service(agency_service_server)
            .serve_with_incoming(stream)
            .await;
        // Server must be running fine...
        assert!(server.is_ok());
        server.unwrap()
    };
    let socket = Arc::clone(&socket);
    // Connect to the server over a Unix socket
    // The URL will be ignored.
    let channel = Endpoint::try_from("http://[::1]")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            let socket = Arc::clone(&socket);
            async move { UnixStream::connect(&*socket).await }
        }))
        .await
        .unwrap();

    let client = AgencyServiceClient::new(channel);

    (server, client)
}
