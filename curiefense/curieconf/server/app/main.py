from curieconf.confserver import app
from curieconf.confserver.backend import Backends
from curieconf.utils.config import (
    CURIECONF_TRUSTED_USERNAME_HEADER,
    CURIECONF_TRUSTED_EMAIL_HEADER,
)

app.backend = Backends.get_backend(app, "git:///cf-persistent-config/confdb")
options = {}
val = CURIECONF_TRUSTED_USERNAME_HEADER
if val:
    options["trusted_username_header"] = val
val = CURIECONF_TRUSTED_EMAIL_HEADER
if val:
    options["trusted_email_header"] = val

app.options = options
