#!/bin/bash

SCRIPT_DIR=$(dirname "${BASH_SOURCE[0]}")
ELASTICSEARCH_URL=${ELASTICSEARCH_URL:-http://elasticsearch:9200/}
KIBANA_URL=${KIBANA_URL:-http://kibana:5601/}
ES_INDEX_NAME=${CURIEFENSE_ES_INDEX_NAME:-curieaccesslog}
ES_PATTERN_PATH="api/saved_objects/index-pattern/${ES_INDEX_NAME}*"

CURL="curl --silent --write-out %{http_code}\n -H Content-Type:application/json --output /dev/null"

wait_for_es () {
	if $CURL -X GET "${ELASTICSEARCH_URL}_cluster/health?wait_for_status=yellow&timeout=10s"|grep -qv 200; then
		sleep 5
		wait_for_es
	fi
}

define_es_index_template() {
	if $CURL "${ELASTICSEARCH_URL}_template/$ES_INDEX_NAME"|grep -q 200; then
		echo "Elastic index template already exists."
	else
		if sed -e "s/INDEX_NAME/$ES_INDEX_NAME/" "$SCRIPT_DIR/index_template.json"|$CURL -X PUT -d @- "${ELASTICSEARCH_URL}_template/$ES_INDEX_NAME"|grep -q 200; then
			echo "Elastic index template created"
		else
			echo "Elastic index template creation failed, retrying."
			sleep 5
			define_es_index_template
		fi
	fi

}

define_es_initial_index () {
	if $CURL "$ELASTICSEARCH_URL$ES_INDEX_NAME-000001"|grep -q 200; then
		echo "Elastic index already exists."
	else
		if sed "s/INDEX_NAME/$ES_INDEX_NAME/" "$SCRIPT_DIR/es_index.json"|$CURL -X PUT -d @- "$ELASTICSEARCH_URL$ES_INDEX_NAME-000001"|grep -q 200; then
			echo "Elastic index and alias created."
		else
			echo "Elastic index and alias creation failed, retrying."
			sleep 5
			define_es_index_mapping
		fi
	fi
}

create_kibana_index_pattern () {
	# Wait for kibana to become reachable
	while true; do
		if $CURL "${KIBANA_URL}api/status"|grep -q 200; then
			break
		fi
		echo "Kibana at $KIBANA_URL is not reachable yet, waiting 5s..."
		sleep 5
	done

	# Check whether the index pattern already exists
	if $CURL "$KIBANA_URL$ES_PATTERN_PATH"|grep -q 200; then
		# already exists
		echo "Kibana index already exists."
	else
		# Create the index pattern
		if $CURL -X POST "$KIBANA_URL$ES_PATTERN_PATH" -H 'kbn-xsrf: true' -d '{"attributes": {"title": "'"$ES_INDEX_NAME*"'","timeFieldName": "@timestamp"}}'|grep -q 200; then

			echo "Kibana index $ES_INDEX_NAME created."
		else
			echo "Kibana index $ES_INDEX_NAME creation failed, retrying."
			sleep 5
			create_kibana_index_pattern
		fi
	fi
}


# in case logs are saved in elasticsearch and not postgres
>&2 echo "Creating an index pattern in Kibana if needed."
wait_for_es
define_es_index_template
define_es_initial_index
if [ -z "$SKIP_KIBANA_INIT" ]; then
	create_kibana_index_pattern
fi
