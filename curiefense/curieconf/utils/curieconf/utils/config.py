import os

CURIECONF_BASE_URL = os.environ.get(
    "CURIECONF_BASE_URL", "http://localhost:5000/api/v3/"
)

CURIECONF_TRUSTED_USERNAME_HEADER = os.environ.get(
    "CURIECONF_TRUSTED_USERNAME_HEADER", ""
)
CURIECONF_TRUSTED_EMAIL_HEADER = os.environ.get("CURIECONF_TRUSTED_EMAIL_HEADER", "")

CURIECONF_HOST = os.environ.get("CURIECONF_HOST", "127.0.0.1")
CURIECONF_PORT = os.environ.get("CURIECONF_PORT", "5000")
CURIECONF_GIT_SSH_KEY_PATH = os.environ.get("CURIECONF_GIT_SSH_KEY_PATH")

CURIE_S3_ACCESS_KEY = os.environ.get("CURIE_S3_ACCESS_KEY")
CURIE_S3_SECRET_KEY = os.environ.get("CURIE_S3_SECRET_KEY")
CURIE_S3CFG_PATH = os.environ.get("CURIE_S3CFG_PATH")

CURIE_MINIO_HOST = os.environ.get("CURIE_MINIO_HOST", "minio")
CURIE_MINIO_ACCESS_KEY = os.environ.get("CURIE_MINIO_ACCESS_KEY")
CURIE_MINIO_SECRET_KEY = os.environ.get("CURIE_MINIO_SECRET_KEY")
CURIE_MINIOCFG_PATH = os.environ.get("CURIE_MINIOCFG_PATH")
CURIE_MINIO_SECURE = os.environ.get("CURIE_MINIO_SECURE", "TRUE")

GOOGLE_APPLICATION_CREDENTIALS = os.environ.get(
    "GOOGLE_APPLICATION_CREDENTIALS", "/var/run/secrets/gs/gs.json"
)

SWAGGER_BASE_PATH = os.environ.get("SWAGGER_BASE_PATH", "/api/v3/")

AUDIT_LOGS_RETENTION_MONTHS = int(os.environ.get("AUDIT_LOGS_RETENTION_MONTHS", "3"))