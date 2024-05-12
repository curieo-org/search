# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

import app.grpc_types.agency_pb2 as agency__pb2


class AgencyServiceStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.pubmed_bioxriv_web_search = channel.unary_unary(
            "/agency.AgencyService/pubmed_bioxriv_web_search",
            request_serializer=agency__pb2.SearchRequest.SerializeToString,
            response_deserializer=agency__pb2.SearchResponse.FromString,
        )


class AgencyServiceServicer(object):
    """Missing associated documentation comment in .proto file."""

    def pubmed_bioxriv_web_search(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details("Method not implemented!")
        raise NotImplementedError("Method not implemented!")


def add_AgencyServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
        "pubmed_bioxriv_web_search": grpc.unary_unary_rpc_method_handler(
            servicer.pubmed_bioxriv_web_search,
            request_deserializer=agency__pb2.SearchRequest.FromString,
            response_serializer=agency__pb2.SearchResponse.SerializeToString,
        ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
        "agency.AgencyService", rpc_method_handlers
    )
    server.add_generic_rpc_handlers((generic_handler,))


# This class is part of an EXPERIMENTAL API.
class AgencyService(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def pubmed_bioxriv_web_search(
        request,
        target,
        options=(),
        channel_credentials=None,
        call_credentials=None,
        insecure=False,
        compression=None,
        wait_for_ready=None,
        timeout=None,
        metadata=None,
    ):
        return grpc.experimental.unary_unary(
            request,
            target,
            "/agency.AgencyService/pubmed_bioxriv_web_search",
            agency__pb2.SearchRequest.SerializeToString,
            agency__pb2.SearchResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
        )
