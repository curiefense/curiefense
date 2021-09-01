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
        "cloudstorage [amazon, google, local]",
        "pydash==5.0.2",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
    ],
)
