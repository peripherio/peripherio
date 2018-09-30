#!/usr/bin/env python

from setuptools import setup, find_packages
from setuptools.command.install import install
from subprocess import check_call

class PostInstallCommand(install):
    def run(self):
        check_call('protoc --python_out=peripherio --grpc_out=peripherio --plugin=protoc-gen-grpc=`which grpc_python_plugin` --proto_path=../protos/ peripherio.proto'.split())
        install.run(self)


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
        },
        )
