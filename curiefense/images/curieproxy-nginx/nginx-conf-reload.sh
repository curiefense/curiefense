#! /bin/bash
  source_file="/cf-config/current/config/customconf.tar.gz"
  target_dir="/etc/nginx/conf.d"
  temp_dir="/tmp/current-conf"
  mkdir -p ${temp_dir}
  rm -rf ${temp_dir}/*
  cp -a ${target_dir}/. ${temp_dir}/
  rm -rf ${target_dir}/*
  logger "Extract $source_file into $target_dir"
  tar xzf ${source_file} -C ${target_dir}
  ## test nginx conf
  logger "Test Nginx"
  nginx -t
  ## check exit code of previous command
  retVal=$?
  if [ $retVal -ne 0 ];
  then
    logger "Nginx failed, restore config files"
    ## restore previously backed-up files
    cp -a  ${temp_dir}/. ${target_dir}/
  else
    ## reload nginx
    nginx -s reload
    logger "Nginx reloaded with the new config"
  fi
