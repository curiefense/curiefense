#!/usr/bin/env python

from setuptools import setup

setup(
    name="curieconf_server",
    version="1.0",
    description="Curiefense configuration server",
    author="Reblaze",
    author_email="phil@reblaze.com",
    packages=[
        "curieconf.confserver",
        "curieconf.confserver.backend",
        "curieconf.confserver.v1",
    ],
    package_data={"curieconf.confserver": ["json/*.schema"]},
    scripts=["bin/curieconf_server"],
    install_requires=[
        "wheel",
        "flask",
        "flask_cors",
        "flask_pymongo",
        "flask-restplus",
        "werkzeug==0.16.1",
        "gitpython",
        "colorama",
        "jmespath",
        "fasteners",
        "jsonpath-ng",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
    ],
)
