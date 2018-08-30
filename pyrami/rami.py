import grpc
import msgpack

import main_pb2
import main_pb2_grpc

channel = grpc.insecure_channel('localhost:50051')
stub = main_pb2_grpc.RamiStub(channel)

def convconf(conf):
    return (main_pb2.Config.Pair(key=k, value=msgpack.packb(v)) for (k,v) in conf.items())

def find_device(category, config={}):
    p_spec = main_pb2.DriverSpecification(category=category)
    p_conf = main_pb2.Config(config=list(convconf(config)))
    response = stub.Find(main_pb2.FindRequest(config=p_conf, spec=p_spec))
    return [Device(r.id) for r in response.results]

class Device(object):
    def __init__(self, device_id):
        self.device_id = device_id

    def __getattr__(self, name):
        def __internal(args):
            ret = stub.Dispatch(main_pb2.DispatchRequest(device=self.device_id, command=name, args=msgpack.packb(args)))
            return msgpack.unpackb(ret.rets)
        return __internal
