#!/usr/bin/env python

from setuptools import setup, find_packages
from setuptools.command.install import install
from setuptools.command.egg_info import egg_info
from subprocess import check_call, check_output

def generate_stub():
    plugin = check_output(['which', 'grpc_python_plugin']).decode()
    check_call('protoc --python_out=peripherio --grpc_out=peripherio --plugin=protoc-gen-grpc={} --proto_path=../protos/ peripherio.proto'.format(plugin).split())

class PostInstallCommand(install):
    def run(self):
        generate_stub()
        install.run(self)

class PostEggInfoCommand(egg_info):
    def run(self):
        generate_stub()
        egg_info.run(self)


setup(name='peripherio',
        version='0.0.1',
        description='Abstracted peripheral interface access',
        author='coord.e',
        author_email='me@coord-e.com',
        url='https://github.com/peripherio/peripherio',
        install_requires=['grpcio-tools==1.14.2', 'grpcio>=1.14.2', 'msgpack>=0.5.6'],
        packages=find_packages(),
        cmdclass={
            'install': PostInstallCommand,
            'egg_info': PostEggInfoCommand,
        },
        )
