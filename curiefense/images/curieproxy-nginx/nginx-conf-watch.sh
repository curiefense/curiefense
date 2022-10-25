#! /bin/bash
filename=/cf-config/current/config/customconf.tar.gz

logger "Calling reload script"
/bin/bash < /usr/local/bin/nginx-conf-reload.sh &

logger "Start watching $source_file"
while true
do
    file_age=$(($(date +%s) - $(date +%s -r "$filename")))
    logger "File age in sec: $file_age"
    if (( file_age < 20 ));
    then
        logger "The file $source_file updated try nginx reload with the new config"
        /usr/local/bin/nginx-conf-reload.sh &
    fi
    sleep 20;
done