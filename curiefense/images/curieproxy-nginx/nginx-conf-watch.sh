#! /bin/bash
filename=/cf-config/current/config/customconf.tar.gz

echo "Calling reload script"
/bin/bash < /usr/local/bin/nginx-conf-reload.sh &

echo "Start watching $source_file"
while true
do
    if [ -f "$filename" ]; then
        file_age=$(($(date +%s) - $(date +%s -r "$filename")))
        echo "File age in sec: $file_age"
        if (( file_age < 20 ));
        then
            echo "The file $source_file updated try nginx reload with the new config"
            /usr/local/bin/nginx-conf-reload.sh &
        fi
    else
	      echo "custom.tar.gz missing"
    fi
    sleep 20;
done
