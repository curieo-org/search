from concurrent import futures
import grpc
from app.grpc_types.rag_pb2_grpc import add_RagServiceServicer_to_server
from app.api import Search
import asyncio

async def serve():
    server = grpc.aio.server(futures.ThreadPoolExecutor(max_workers=10))
    add_RagServiceServicer_to_server(Search(), server)
    port = 50051
    server.add_insecure_port(f"[::]:{port}")
    await server.start()
    print("Server started")
    await server.wait_for_termination()

def start_server():
    asyncio.run(serve())

if __name__ == "__main__":
    start_server()