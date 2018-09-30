import grpc
import msgpack

from peripherio.protos import peripherio_pb2
from peripherio.protos import peripherio_pb2_grpc

import contextlib


@contextlib.contextmanager
def connect(uri='localhost:57601'):
    with grpc.insecure_channel(uri) as channel:
        stub = peripherio_pb2_grpc.PeripherioStub(channel)
        conn = Connection(stub)
        try:
            yield conn
        finally:
            channel.close()


def convconf(conf):
    return (peripherio_pb2.Config.Pair(key=k, value=msgpack.packb(v))
            for (k, v) in conf.items())


class Connection(object):
    def __init__(self, stub):
        self.stub = stub

    def find_device(self, category, config={}):
        p_spec = peripherio_pb2.DriverSpecification(category=category)
        p_conf = peripherio_pb2.Config(config=list(convconf(config)))
        response = self.stub.Find(
            peripherio_pb2.FindRequest(config=p_conf, spec=p_spec))
        return [Device(r.id, r.display_name, self) for r in response.results]

    def list_device(self, config={}):
        p_spec = peripherio_pb2.DriverSpecification()
        p_conf = peripherio_pb2.Config(config=list(convconf(config)))
        response = self.stub.Find(
            peripherio_pb2.FindRequest(config=p_conf, spec=p_spec))
        return [Device(r.id, r.display_name, self) for r in response.results]


class Device(object):
    def __init__(self, device_id, name, conn):
        self.device_id = device_id
        self.name = name
        self.conn = conn

    def __getattr__(self, name):
        def __internal(args={}):
            ret = self.conn.stub.Dispatch(peripherio_pb2.DispatchRequest(
                device=self.device_id, command=name, args=msgpack.packb(args)))
            return msgpack.unpackb(ret.rets)
        return __internal
