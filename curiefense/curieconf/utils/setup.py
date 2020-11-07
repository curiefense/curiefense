#!/usr/bin/env python

from setuptools import setup

setup(
    name='curieconf_utils',
    version='1.0',
    description='Curiefense configuration utils',
    author='Reblaze',
    author_email='phil@reblaze.com',
    packages=['curieconf.utils'],

    install_requires=[
        "wheel",
        "azure==4.0.0",  # fix to force azure version before cloudstorage[microsoft]
        "cloudstorage [amazon, google, local, microsoft, minio, rackspace]",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
    ]
)

