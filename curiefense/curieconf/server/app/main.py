from curieconf.confserver import app
from curieconf.confserver.backend import Backends

app.backend = Backends.get_backend(app, "git:///config/confdb")
