#!/usr/bin/env python

from setuptools import setup

setup(
    name="curieconf_server",
    version="1.1",
    description="Curiefense configuration server",
    author="Reblaze",
    author_email="phil@reblaze.com",
    packages=[
        "curieconf.confserver",
        "curieconf.confserver.backend",
        "curieconf.confserver.v1",
        "curieconf.confserver.v2",
    ],
    package_data={
        "curieconf.confserver": [
            "json/*.schema",
            "v1/json/*.schema",
            "v2/json/*.schema",
        ]
    },
    scripts=["bin/curieconf_server"],
    install_requires=[
        "wheel",
        "flask",
        "flask_cors",
        "flask_pymongo",
        "flask-restx",
        "werkzeug==0.16.1",
        "gitpython",
        "colorama",
        "jmespath",
        "fasteners",
        "jsonpath-ng",
        "pydash==5.0.2",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
    ],
)
