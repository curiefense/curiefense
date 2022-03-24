#!/usr/bin/env python3

# Python requirements: pytest requests
# install curieconfctl:
# (cd ../curiefense/curieconf/utils ; pip3 install .)
# (cd ../curiefense/curieconf/client ; pip3 install .)
#
# To run this with minikube (does not support IPv6):
#
# pytest -m all_modules --base-protected-url http://$(minikube ip):30081 --base-conf-url http://$(minikube ip):30000/api/v2/ --base-ui-url http://$(minikube ip):30080 --elasticsearch-url http://$IP:30200 .      # pylint: disable=line-too-long
#
# To run this with docker-compose:
# pytest -m all_modules --base-protected-url http://localhost:30081/ --base-conf-url http://localhost:30000/api/v2/ --base-ui-url http://localhost:30080 --elasticsearch-url http://localhost:9200 .      # pylint: disable=line-too-long

# pylint: disable=too-many-lines,too-many-public-methods
# pylint: disable=too-many-arguments,too-few-public-methods,too-many-statements
# pylint: disable=missing-function-docstring,missing-module-docstring
# pylint: disable=missing-class-docstring

# This is not really a problem for fixtures
# pylint: disable=redefined-outer-name

# This is often wrong: fixtures are not mentioned in the function, but they
# define the required test environment
# pylint: disable=unused-argument

# This follows examples from the pytest doc: tests are class methods, even
# though they don't use self


##################################################################################################################
# To run e2e test on your instance/local machine:

# 1 - Set your Ip/localhost in default values in conftest.py

# 2 - From e2e folder in terminal, execute:
#
#     pytest -m module_name --html=./reports/report_name.html --self-contained-html
#
# 3 - Module names appear in pytest.ini file
# 4 - Report will be automatically generated in reports folder
# 5 - V1/V2 adjustments are made automatically