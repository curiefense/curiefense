#! /bin/bash

###########################################################################
# this file downloads the last hour's 36th minute file from GS bucket     #
# of given planets. convert them into the ES format, plus adding          #
# the _bulk index line (2 lines per document).                            #
# split the output into a 20K lines each (50MB seems to be the threshold) #
# insert the split files into Elasticsearch                               #
###########################################################################

PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin"
HOME="/home/tzury/"
BOTO_CONFIG="/home/tzury/.config/gcloud/legacy_credentials/tzury@reblaze.com/.boto"
wd="/home/tzury/bqmigrate"

## pick last hour's 36's minute. e.g. 20210131T0536
timespamp=$(date -d "1 hour ago" "+%Y%m%dT%H36")
logger fetch-convert-split-push $wd $planets $timestamp

cd $wd || exit 1

for planet in "$@";
do
    logger fetch-convert-split-push $planet
    /snap/bin/gsutil cp "gs://rbz-logs-$planet/bqinsert/queue-in/compose/$timespamp-bq_load00" "$wd/bq-json/$planet$timespamp-bq_load00" >> /var/log/syslog;
done

for fi in $wd/bq-json/*-bq_load00;
do
    logger fetch-convert-split-push $fi;
    /usr/bin/python3 $wd/utils/convert.py < $fi | split -l 20000 - "$wd/bq-json/$(basename $fi).split." -da 4  >> /var/log/syslog;
done

for fi in $wd/bq-json/*.split.*;
do
    logger fetch-convert-split-push $fi
    /usr/bin/curl -s -H "Content-Type: application/x-ndjson" -XPOST localhost:9200/_bulk --data-binary "@$fi";
done

## cleanup
logger fetch-convert-split-push cleaning up
rm $wd/bq-json/*

