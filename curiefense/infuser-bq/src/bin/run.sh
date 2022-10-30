#!/usr/bin/env bash
# Read the environment variables for the main process (PID 1) running in the Docker container:
export $(xargs -0 -a "/proc/1/environ")

/usr/local/bin/infuserbqctl