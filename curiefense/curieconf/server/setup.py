#!/usr/bin/env python

from setuptools import setup

setup(
    name="curieconf_server",
    version="3.0",
    description="Curiefense configuration server",
    author="Reblaze",
    author_email="phil@reblaze.com",
    packages=[
        "curieconf.confserver",
        "curieconf.confserver.backend",
        "curieconf.confserver.v3",
    ],
    package_data={
        "curieconf.confserver": [
            "v3/json/*.schema",
        ]
    },
    scripts=["bin/curieconf_server"],
    install_requires=[
        "wheel",
        "flask==2.2.3",
        "flask_cors==3.0.10",
        "flask-restx==1.1.0",
        "markupsafe==2.1.2",
        "werkzeug==2.2.3",
        "gitpython==3.1.31",
        "colorama",
        "jmespath",
        "fasteners",
        "jsonpath-ng==1.5.3",
        "fastapi==0.94.0",
        "prometheus-fastapi-instrumentator==5.11.1",
        "pydantic==1.10.6",
        "uvicorn==0.21.0",
        "bleach==6.0.0",
        "prometheus-flask-exporter==0.22.3",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
    ],
)
