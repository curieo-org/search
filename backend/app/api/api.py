import grpc

from .search import Search
from app.grpc_types.rag_pb2_grpc import add_RagServiceServicer_to_server


def setup_grpc_api(server: grpc.Server):
    add_RagServiceServicer_to_server(Search(), server)