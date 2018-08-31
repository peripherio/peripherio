#!/usr/bin/env python

from distutils.core import setup

setup(name='rami',
        version='0.0.1',
        description='Abstracted peripheral interface access',
        author='coord.e',
        author_email='me@coord-e.com',
        url='https://github.com/coord-e/rami',
        install_requires=['grpcio-tools=1.14.2', 'grpcio>=1.14.2', 'msgpack>=0.5.6'],
        packages=find_packages(),
        )