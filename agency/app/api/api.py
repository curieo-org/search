import grpc

from app.grpc_types.agency_pb2_grpc import add_AgencyServiceServicer_to_server

from .search import Search


def setup_grpc_api(server: grpc.Server):
    add_AgencyServiceServicer_to_server(Search(), server)
