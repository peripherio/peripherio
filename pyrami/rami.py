import grpc
import msgpack

import main_pb2
import main_pb2_grpc


def run():
    with grpc.insecure_channel('localhost:50051') as channel:
        stub = main_pb2_grpc.RamiStub(channel)
        response = stub.List(main_pb2.Config(config=[main_pb2.Config.Pair(key="if.type", value=msgpack.packb("i2c"))]))
    print("Greeter client received: " + str(response.results))


if __name__ == '__main__':
    run()
