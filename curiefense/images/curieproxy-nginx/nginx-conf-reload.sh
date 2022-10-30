#! /bin/bash
pid_file="/usr/local/openresty/nginx/logs/nginx.pid"
source_file="/cf-config/current/config/customconf.tar.gz"
target_dir="/etc/nginx/conf.d"
temp_dir="/tmp/current-conf"
if [ ! -f "$source_file" ]; then
    echo "$source_file does not exist."
    exit 0
fi
mkdir -p ${temp_dir}
[ ! -d "/lua/customcode" ] && mkdir -p /lua/customcode
rm -rf ${temp_dir}/*
cp -a ${target_dir}/. ${temp_dir}/
rm -rf ${target_dir}/*
echo "Extract $source_file into $target_dir"
tar xzf ${source_file} -C ${target_dir}
## test nginx conf
echo "Test Nginx"
nginx -t
## check exit code of previous command
retVal=$?
if [ $retVal -ne 0 ];
then
  echo "Nginx failed, restore config files"
  ## restore previously backed-up files
  cp -a  ${temp_dir}/. ${target_dir}/
  cp ${targer_dir}/lua/customcode.lua /lua/customcode/lua.lua
else
  ## reload nginx
  n=0
  until [ "$n" -ge 10 ]
  do
    p_id=$(cat $pid_file)
    [ -n "$p_id" ]  && [ "$p_id" -gt 0 ] && echo "NGINX started" && break
    echo nginx not started
    n=$((n+1))
    sleep 1
  done
  if  [ -e $pid_file ] && p_id=$(cat $pid_file) && [ -n "$p_id" ]  && [ "$p_id" -gt 0 ] && ps "$p_id" > /dev/null
  then
    echo reloading nginx
    nginx -s reload
    echo "Nginx reloaded with the new config"
  fi
fi
