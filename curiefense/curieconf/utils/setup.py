#!/usr/bin/env python

from setuptools import setup

setup(
    name="curieconf_utils",
    version="1.0",
    description="Curiefense configuration utils",
    author="Reblaze",
    author_email="phil@reblaze.com",
    packages=["curieconf.utils"],
    install_requires=[
        "wheel",
        "google-crc32c==1.5.0",
        "minio==7.1.13",
        "cloudstorage [amazon, google, local, minio]==0.11.0",
        "pydash==5.1.2",
        "MarkupSafe==2.1.2",
        "flask==2.2.2",
        "flask-restx==1.0.5",
        "werkzeug==2.2.2",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
    ],
)
