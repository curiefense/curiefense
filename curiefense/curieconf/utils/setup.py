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
        "google-crc32c==1.1.2",
        "minio==6.0.2",
        "cloudstorage [amazon, google, local, minio]==0.10.1",
        "pydash==5.0.2",
        "MarkupSafe==2.0.1",
        "flask==2.1.2",
        "flask-restx==0.5.1",
        "werkzeug==2.1.2",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
    ],
)
