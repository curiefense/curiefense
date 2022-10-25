#! /bin/bash
  source_file="/cf-config/current/config/customconf.tar.gz"
  target_dir="/etc/nginx/conf.d"
  temp_dir="/tmp/current-conf"
  mkdir -p ${temp_dir}
  mkdir -p /lua/customcode
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
    nginx -s reload
    echo "Nginx reloaded with the new config"
  fi
