import grpc

from .search import Search
from app.grpc_types.agency_pb2_grpc import add_AgencyServiceServicer_to_server


def setup_grpc_api(server: grpc.Server):
    add_AgencyServiceServicer_to_server(Search(), server)
