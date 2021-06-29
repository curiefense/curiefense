#!/bin/sh
redis-server --loglevel notice &
export REDIS_HOST=127.0.0.1
luajit /home/builder/test.lua
