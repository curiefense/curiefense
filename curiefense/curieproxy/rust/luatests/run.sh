#!/bin/sh
redis-server --loglevel notice &
export REDIS_HOST=127.0.0.1
/usr/local/bin/luajit /home/builder/test.lua