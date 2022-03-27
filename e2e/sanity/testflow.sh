#! /bin/bash


if [ -z "$CURIETARGET" ]
then
      echo "\$CURIETARGET is empty. exiting..."
      exit 0
fi

RED='\033[0;31m'
NC='\033[0m' # No Color
B=$(tput bold)
N=$(tput sgr0)

target="$CURIETARGET:30081"
ts=$(date +'%Y%m%d%H%M')
baseurl0="http://${target}"
baseurl="http://${target}/${ts}"


function test01 {

	printf "\n\n${B}GLOBAL FILTER + ACTION\t${N}\n"

	printf "\n01:\t 301\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}"/ --data "foobar=baz"
	printf "\n02:\t 301\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}"/ -H "foobar:baz"
}

function test02 {

	printf "\n\n${B}GLOBAL FILTER + ACL ${N}\n"

	printf "\n03:\t 200\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}"/acl/passthrough-bar --data "foo=bar&zoo=/etc/passwd"
	printf "\n04:\t 247\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}"/acl/deny-bot-baz --data "a=1" -H "foo:bar"
	printf "\n05:\t 200\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}"/acl/allow-foobar --data "foo=bar"
	printf "\n06:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}"/acl/enforce-deny-foobar -H "foo:bar"
}

function test03 {

	printf "\n\n${B}CONTENT FILTER${N}\n"

	printf "\n07:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/1" --data "a=1" -H "test: cf-risk-level-1"
	printf "\n08:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/2" --data "a=2" -H "test: cf-risk-level-2"
	printf "\n09:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/3" --data "a=3" -H "test: cf-risk-level-3"
	printf "\n10:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/4" --data "a=4" -H "test: cf-risk-level-4"
	printf "\n11:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/5" --data "a=5" -H "test: cf-risk-level-5"


	printf "\n12:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/1/plus" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","c":"cf-risk-level-3","d":"cf-risk-level-4","e":"cf-risk-level-5"}'
	printf "\n13:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/2/plus" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","c":"cf-risk-level-3","d":"cf-risk-level-4","e":"cf-risk-level-5"}'
	printf "\n14:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/3/plus" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","c":"cf-risk-level-3","d":"cf-risk-level-4","e":"cf-risk-level-5"}'
	printf "\n15:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/4/plus" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","c":"cf-risk-level-3","d":"cf-risk-level-4","e":"cf-risk-level-5"}'
	printf "\n16:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/5/plus" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","c":"cf-risk-level-3","d":"cf-risk-level-4","e":"cf-risk-level-5"}'


	printf "\n17:\t 200\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/1/minus" -H "content-type: application/json" --data '{"b":"cf-risk-level-2","c":"cf-risk-level-3","d":"cf-risk-level-4","e":"cf-risk-level-5"}'
	printf "\n18:\t 200\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/2/minus" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","c":"cf-risk-level-3","d":"cf-risk-level-4","e":"cf-risk-level-5"}'
	printf "\n19:\t 200\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/3/minus" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","d":"cf-risk-level-4","e":"cf-risk-level-5"}'
	printf "\n20:\t 200\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/4/minus" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","c":"cf-risk-level-3","e":"cf-risk-level-5"}'
	printf "\n21:\t 200\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/5/minus" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","c":"cf-risk-level-3","d":"cf-risk-level-4"}'


	printf "\n22:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cfr/linj/both" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","c":"cf-risk-level-3","d":"cf-risk-level-4","e":"cf-risk-level-5", "f": "%3Cscript%3Edocument.body.innerHTML="}'
	printf "\n23:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cfr/linj/both" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","c":"cf-risk-level-3","d":"cf-risk-level-4","e":"cf-risk-level-5", "f": "- /* foo */ ( /* bar */ -SELECT 1 );"}'

	printf "\n24:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cfr/linj/ex-xss" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","c":"cf-risk-level-3","d":"cf-risk-level-4","e":"cf-risk-level-5", "f": "%3Cscript%3Edocument.body.innerHTML="}'
	printf "\n25:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cfr/linj/xss" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","c":"cf-risk-level-3","d":"cf-risk-level-4","e":"cf-risk-level-5", "f": "%3Cscript%3Edocument.body.innerHTML="}'

	printf "\n26:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cfr/linj/sqli" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","c":"cf-risk-level-3","d":"cf-risk-level-4","e":"cf-risk-level-5", "f": "- /* foo */ ( /* bar */ -SELECT 1 );"}'
	printf "\n27:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cfr/linj/cfr-linj-ex-sqli" -H "content-type: application/json" --data '{"a":"cf-risk-level-1","b":"cf-risk-level-2","c":"cf-risk-level-3","d":"cf-risk-level-4","e":"cf-risk-level-5", "f": "- /* foo */ ( /* bar */ -SELECT 1 );"}'

	printf "\n28:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/maskignore" -H "content-type: application/json" --data '{"ignorename": "- /* foo */ ( /* bar */ -SELECT 1 );", "14ignoreregex23": "<foo  bar =  >", "maskname": "can you see me?", "maskregex12": "I hope you cannot", "nomasking": "clear"}'; printf "\t(verify logs sigs and masking)"
	printf "\n29:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/maskignore" -H "content-type: application/json" --data '{"14ignoreregex23": "- /* foo */ ( /* bar */ -SELECT 1 );", "ignorename": "<foo  bar =  >", "maskname": "can you see me?", "maskregex12": "I hope you cannot", "nomasking": "clear"}'; printf "\t(verify logs sigs and masking)"


	printf "\n30:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/mask-ignore-negation" -H "content-type: application/json" --data '{"foo": "- /* foo */ ( /* bar */ -SELECT 1 );", "bar": "<foo  bar =  >", "baz": "can you see me?", "creditcard": "I hope you cannot", "creditcardnum": "also-hidden"}'; printf "\t(verify logs sigs and masking)"

	printf "\n31:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/ctype/xml" -H "content-type: application/json" --data '{"foo": "Zoo}';
	printf "\n32:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/ctype/xml" -H "content-type: application/xml" --data '{"foo": "Zoo}';
	printf "\n33:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/ctype/xml" --data '{"foo": "Zoo}';
	printf "\n34:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/ctype/xml" -H "content-type: application/xml" --data '<?xml version="1.0" encoding="UTF-8"?><note><body>Don not forget me this weekend!</body></no-note>'
	printf "\n35:\t 200\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/ctype/xml" -H "content-type: application/xml" --data '<?xml version="1.0" encoding="UTF-8"?><note><body>Don not forget me this weekend!</body></note>'

	printf "\n36:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/ctype/json" -H "content-type: application/xml" --data '{"foo": "Zoo}';
	printf "\n37:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/ctype/json" --data '{"foo": "Zoo}';
	printf "\n38:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/ctype/json" -H "content-type: application/xml" --data '<?xml version="1.0" encoding="UTF-8"?><note><body>Don not forget me this weekend!</body></note>'
	printf "\n39:\t 403\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/ctype/json" -H "content-type: application/json" --data '{"foo": "Zoo}';
	printf "\n40:\t 200\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/ctype/json" -H "content-type: application/json" --data '{"foo": "Zoo"}';


	printf "\n41:\t 200\t" && curl -s -o /dev/null -w "%{http_code}" "${baseurl}/cf/risk/5/report" --data "a=5" -H "test: cf-risk-level-5"

}


function test04 {

	printf "\n\n${B}RATE LIMIT${N}\n"

	echo "-- 3:6:9-BY-IP-OVER-30"
	i=1
	until [ $i -gt 20 ]
	do
		printf '%3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" "${baseurl}/rl/er/369-by-ip-over-30"; echo;
		((i=i+1))
		# sleep 1
	done
}

function test05 {

	echo "-- 3:6:9-BY-HEADER-OVER-30 (X2 UNIQUE HEADERS)"
	i=1
	until [ $i -gt 20 ]
	do
		printf 'a %3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" "${baseurl}/rl/er/369-by-header-over-30" -H "x-limit-id: 1"; echo;
		printf 'b %3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" "${baseurl}/rl/er/369-by-header-over-30" -H "x-limit-id: 2"; echo;
		((i=i+1))
		# sleep 0.9
	done
}

function test06 {

	echo "-- 3:6:BAN-BY-ARG-OVER-30 (X2 UNIQUE ARGS)"

	i=1
	until [ $i -gt 20 ]
	do
		printf 'a %3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" "${baseurl}/rl/er/36ban-by-arg-over-30" --data "x-arg-ban=my@email.com"; echo;
		printf 'b %3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" "${baseurl}/rl/er/36ban-by-arg-over-30" --data "x-arg-ban=my@email.net"; echo;
		((i=i+1))
		# sleep 0.9
	done
}

function test07 {

	echo "-- 3:6:BAN-BY-COOKIE-OVER-30 (x2 UNIQUE COOKIES)"

	i=1
	until [ $i -gt 20 ]
	do
		printf 'a %3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" "${baseurl}/rl/er/369-by-cookie-over-30" --cookie "user=1234"; echo;
		printf 'b %3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" "${baseurl}/rl/er/369-by-cookie-over-30" --cookie "user=2345"; echo;
		((i=i+1))
		# sleep 0.9
	done
}



function test08 {

	echo "-- RL a:cAeA b:cCeA c:cHeA d:cAeA2 e:cAeC f:cAeH"

	i=1
	until [ $i -gt 25 ]
	do

		printf 'a %3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" "${baseurl}/xrl/cAeA" --data "userid=me1@email.com&password=$RANDOM"; echo;
		printf 'b %3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" "${baseurl}/xrl/cCeA" --cookie "userid=me1@email.com" --data "password=$RANDOM"; echo;
		printf 'c %3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}"  -H "userid: me1@email.com" "${baseurl}/xrl/cHeA" --data "password=$RANDOM"; echo;

		printf 'd %3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" "${baseurl}/xrl/cAeA2" --data "userid=$RANDOM@email.org&password=abcd1q2w3e1"; echo;
		printf 'e %3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" "${baseurl}/xrl/cAeC" --cookie "userid=$RANDOM@email.com" --data "password=abcd1q2w3e1"; echo;
		printf 'f %3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}"  -H "userid: $RANDOM" "${baseurl}/xrl/cAeH" --data "password=abcd1q2w3e1"; echo;

		((i=i+1))

	done

}

function test09 {

	printf "\n\n${B}EXCLUDED FROM RATE LIMIT (foo:bar)${N}\n"

	echo "-- 3:6:9-BY-IP-OVER-30"
	i=1
	until [ $i -gt 20 ]
	do
		printf '%3.3d' "${i}"; printf " - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -H "foo: bar" "${baseurl}/rl/er/369-by-ip-over-30"; echo;
		((i=i+1))
		# sleep 1
	done
}

function test10 {
	## GET POST PUT DELETE DELETE DELETE...

	printf "\n\n${B}FLOW CONTROL 503 -- GET POST PUT DELETE DELETE DELETE...${N}\n"

	printf "   GET - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X GET    	-H "host: www.example.com" "${baseurl0}/flow/503/step-01"; echo;
	printf "  POST - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X POST    	-H "host: www.example.net" "${baseurl0}/flow/503/step-02"; echo;
	printf "   PUT - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X PUT    	-H "host: www.example.com" "${baseurl0}/flow/503/step-03"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/503/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/503/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/503/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/503/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/503/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/503/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/503/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/503/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/503/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/503/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/503/step-04"; echo;
	echo "...."
	sleep 10;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/503/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/503/step-04"; echo;

}


function test11 {
	## GET POST PUT DELETE DELETE DELETE...
	printf "\n\n${B}FLOW CONTROL LEGIT -- GET POST PUT DELETE DELETE DELETE...${N}\n"

	printf "   GET - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X GET    	-H "host: www.example.com" "${baseurl0}/flow/ban24/step-01"; echo;
	printf "  POST - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X POST    	-H "host: www.example.com" "${baseurl0}/flow/ban24/step-02"; echo;
	printf "   PUT - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X PUT    	-H "host: www.example.com" "${baseurl0}/flow/ban24/step-03"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
}

function test12 {
	## GET POST PUT DELETE DELETE DELETE...
	printf "\n\n${B}FLOW CONTROL BAN -- GET POST PUT DELETE DELETE DELETE...${N}\n"

	printf "   GET - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X GET    	-H "host: www.example.org" "${baseurl0}/flow/ban24/step-01"; echo;
	printf "  POST - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X POST    	-H "host: www.example.com" "${baseurl0}/flow/ban24/step-02"; echo;
	printf "   PUT - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X PUT    	-H "host: www.example.com" "${baseurl0}/flow/ban24/step-03"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	echo "...."
	sleep 14;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	echo "...."
	sleep 10;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
	printf "DELETE - $(date +'%H:%M:%S') - ";curl -s -o /dev/null -w "%{http_code}" -X DELETE   -H "host: www.example.com" "${baseurl0}/flow/ban24/step-04"; echo;
}

test01;
test02;
test03;
test04;
test05;
test06;
test07;
test08;
test09;
test10;
test11;
test12;

echo

