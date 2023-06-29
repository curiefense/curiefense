import os

CURIECONF_BASE_URL = os.environ.get(
    "CURIECONF_BASE_URL", "http://localhost:5000/api/v3/"
)

CURIECONF_TRUSTED_USERNAME_HEADER = os.environ.get(
    "CURIECONF_TRUSTED_USERNAME_HEADER", None
)
CURIECONF_TRUSTED_EMAIL_HEADER = os.environ.get("CURIECONF_TRUSTED_EMAIL_HEADER", None)
