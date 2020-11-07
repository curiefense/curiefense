#!/usr/bin/env python

from distutils.core import setup

setup(name='curielogserver',
      version='1.0',
      description='Curiefense log server',
      author='Reblaze',
      author_email='phil@reblaze.com',
      packages=['curielogserver'],
      scripts=['bin/curielogserver'],
      package_data={'curielogserver': ['*.yaml']},
      install_requires=[
          "wheel",
          "pyyaml",
          "psycopg2",
          "werkzeug==0.16.1",
          "flask",
          "flask_restplus",
      ],
)
