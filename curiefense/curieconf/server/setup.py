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
        "flask==2.1.2",
        "flask_cors==3.0.10",
        "flask_pymongo==2.3.0",
        "flask-restx==0.5.1",
        "markupsafe==2.0.1",
        "werkzeug==2.1.2",
        "gitpython==3.1.30",
        "colorama",
        "jmespath",
        "fasteners",
        "jsonpath-ng==1.5.3",
        "pydash==5.0.2",
        "fastapi==0.87.0",
        "prometheus-fastapi-instrumentator==5.9.1",
        "pydantic==1.10.2",
        "uvicorn==0.19.0",
        "bleach==6.0.0",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
    ],
)
