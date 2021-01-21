#!/bin/bash

INIT_DIR=${INIT_DIR:-/init}
ES_INDEX_NAME=${CURIEFENSE_ES_INDEX_NAME:-curieaccesslog}
ES_PATTERN_PATH="api/saved_objects/index-pattern/${ES_INDEX_NAME}"
USE_DATA_STREAMS=${USE_DATA_STREAMS:-true}

CURL="curl --silent --write-out %{http_code}\n -H Content-Type:application/json --output /dev/null"

wait_for_es () {
	if $CURL -X GET "${ELASTICSEARCH_URL}_cluster/health?wait_for_status=yellow&timeout=10s"|grep -qv 200; then
		sleep 5
        echo $ELASTICSEARCH_URL
		wait_for_es
	fi
}

define_es_lifecycle_policy () {
	if $CURL "${ELASTICSEARCH_URL}_ilm/policy/$ES_INDEX_NAME"|grep -q 200; then
		echo "Elasticsearch lifecycle policy already exists."
	else
		if $CURL -X PUT "${ELASTICSEARCH_URL}_ilm/policy/$ES_INDEX_NAME" --data-binary @$INIT_DIR/es_lifecycle_policy.json|grep -q 200; then
			echo "Elasticsearch lifecycle policy defined."
		else
			echo "Elasticsearch lifecycle policy creation failed, retrying."
			sleep 5
			define_es_lifecycle_policy
		fi
	fi
}

define_es_index_template() {
	if $CURL "${ELASTICSEARCH_URL}_index_template/$ES_INDEX_NAME"|grep -q 200; then
		echo "Elastic index template already exists."
	else
        if [[ "$USE_DATA_STREAMS" == "true" ]];
        then
            DATA_STREAMS='"data_stream": {},'
        fi

		if sed -e "s/INDEX_NAME/$ES_INDEX_NAME/" -e "s/DATA_STREAMS/$DATA_STREAMS/" $INIT_DIR/index_template.json|$CURL -X PUT -d @- "${ELASTICSEARCH_URL}_index_template/$ES_INDEX_NAME"|grep -q 200; then
			echo "Elastic index template created"
		else
			echo "Elastic index template creation failed, retrying."
			sleep 5
			define_es_index_template
		fi
	fi

}

define_es_initial_index () {
    if [[ "$USE_DATA_STREAMS" == "true" ]]; then
		echo "Using datastreams, no need for initial index."
        return
    fi
	if $CURL "$ELASTICSEARCH_URL$ES_INDEX_NAME-000001"|grep -q 200; then
		echo "Elastic index already exists."
	else
		if sed "s/INDEX_NAME/$ES_INDEX_NAME/" $INIT_DIR/es_index.json|$CURL -X PUT -d @- "$ELASTICSEARCH_URL$ES_INDEX_NAME-000001"|grep -q 200; then
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
		if $CURL "${CURIELOGGER_KIBANA_URL}api/status"|grep -q 200; then
			break
		fi
		echo "Kibana at $CURIELOGGER_KIBANA_URL is not reachable yet, waiting 5s..."
		sleep 5
	done

	# Check whether the index pattern already exists
	if $CURL "$CURIELOGGER_KIBANA_URL$ES_PATTERN_PATH"|grep -q 200; then
		# already exists
		echo "Kibana index already exists."
	else
		# Create the index pattern
		if $CURL -X POST "$CURIELOGGER_KIBANA_URL$ES_PATTERN_PATH" -H 'kbn-xsrf: true' -d '{"attributes": {"title": "'"$ES_INDEX_NAME*"'","timeFieldName": "timestamp"}}'|grep -q 200; then

			echo "Kibana index $ES_INDEX_NAME created."
		else
			echo "Kibana index $ES_INDEX_NAME creation failed, retrying."
			sleep 5
			create_kibana_index_pattern
		fi
	fi
}


if [ -n "$CURIELOGGER_INITIALIZE_KIBANA_INDEX" ]; then
	# in case logs are saved in elasticsearch and not postgres
	>&2 echo "Creating an index pattern in Kibana if needed."
	wait_for_es
	define_es_lifecycle_policy
	define_es_index_template
	define_es_initial_index
	create_kibana_index_pattern
fi

/bin/curielogger
