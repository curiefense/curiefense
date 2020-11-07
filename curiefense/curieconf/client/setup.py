#!/usr/bin/env python

from setuptools import setup

setup(
    name='curieconf_client',
    version='1.0',
    description='Curiefense configuration clients',
    author='Reblaze',
    author_email='phil@reblaze.com',
    packages=['curieconf.confclient',
              'curieconf.cli'],
    scripts=['bin/curieconf_cli'],

    install_requires=[
        "wheel",
        "cloudstorage",
        "colorama",
        "pyyaml",
        "simple-rest-client",
        "typer",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
    ]
)
