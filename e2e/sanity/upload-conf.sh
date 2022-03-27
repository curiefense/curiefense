#! /bin/bash

if [ -z "$CURIETARGET" ]
then
      echo "\$CURIETARGET is empty. exiting..."
      exit 0
fi

target="$CURIETARGET:30000"
uitarget="$CURIETARGET:30080"

curl -XDELETE "http://${target}/api/v2/configs/master/d/aclprofiles/"
curl -H "content-type: application/json" -XPOST "http://${target}/api/v2/configs/master/d/aclprofiles/" --data @aclprofiles.json

curl -XDELETE "http://${target}/api/v2/configs/master/d/globalfilters/"
curl -H "content-type: application/json" -XPOST "http://${target}/api/v2/configs/master/d/globalfilters/" --data @globalfilters.json

curl -XDELETE "http://${target}/api/v2/configs/master/d/securitypolicies/"
curl -H "content-type: application/json" -XPOST "http://${target}/api/v2/configs/master/d/securitypolicies/" --data @securitypolicies.json

curl -XDELETE "http://${target}/api/v2/configs/master/d/contentfilterprofiles/"
curl -H "content-type: application/json" -XPOST "http://${target}/api/v2/configs/master/d/contentfilterprofiles/" --data @contentfilterprofiles.json

curl -XDELETE "http://${target}/api/v2/configs/master/d/contentfilterrules/"
curl -H "content-type: application/json" -XPOST "http://${target}/api/v2/configs/master/d/contentfilterrules/" --data @contentfilterrules.json

curl -XDELETE "http://${target}/api/v2/configs/master/d/ratelimits/"
curl -H "content-type: application/json" -XPOST "http://${target}/api/v2/configs/master/d/ratelimits/" --data @ratelimits.json

curl -XDELETE "http://${target}/api/v2/configs/master/d/flowcontrol/"
curl -H "content-type: application/json" -XPOST "http://${target}/api/v2/configs/master/d/flowcontrol/" --data @flowcontrol.json


RED='\033[0;31m'
NC='\033[0m' # No Color


printf "${RED}******\tREMEMBER PUBLISH CHANGES BEFORE RUNNING THE TESTS!\t******${NC}\n"

open http://$uitarget/publish
