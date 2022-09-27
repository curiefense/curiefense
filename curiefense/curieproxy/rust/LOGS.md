# Log format

The log is by default sent to stdout. In the Curieproxy containers, it is piped to *filebeat* that will store it
in ElasticSearch.

The log file is a JSON encoded data structure, where the top level is an object with the following members:

 * `timestamp`: time when the request started to be processed, encoded as a string,
 * `request_id`: unique identifier for the request, currently only provided by envoy,
 * `security_config`: see the *security config* section,
 * `arguments`: an object where the keys are the argument names, and the values argument values,
 * `path`: query path,
 * `path_parts`: an object where the keys are the path part identifiers, and the values are path parts,
 * `authority`: the authority meta data, if it exists, or the *host* header,
 * `cookies`: an object where the keys are the cookie names, and the values are cookie values,
 * `headers`: an object where the keys are the header names, and the values are header values,
 * `tags`: a list of strings, representing the request tags,
 * `uri`: request URI, as a string,
 * `ip`: request IP, as a string,
 * `method`: request method verb, as a string,
 * `response_code`: the response code that was served to the user,
 * `logs`: a list of string, destined for human consumption, with an unspecified format,
 * `processing_stage`: a number representing the stage where the request stopped being processed:
    * 0: initialization, should never happen,
    * 1: security policy level, this means no security policy could be selected,
    * 2: global filter stage,
    * 3: flow control stage,
    * 4: rate limit stage,
    * 5: ACL stage,
    * 6: content filter stage.
 * `trigger_counters`: an object with keys of the form:
    * `TRIGGER`: length of the `TRIGGER` list,
    * `TRIGGER_active`: amount of items in the `TRIGGER` list that would cause a block.
    Here, `TRIGGER` can be `acl`, `global_filters`, `flow_control`, `rate_limit` or `content_filters`.

 * `acl_triggers`: triggers for the `acl` trigger type (see below),
 * `rate_limit_triggers`: triggers for the `rate_limit` trigger type (see below),
 * `flow_control_triggers`: triggers for the `flow_control` trigger type (see below),
 * `global_filter_triggers`: triggers for the `global_filter` trigger type (see below),
 * `content_filter_triggers`: triggers for the `content_filter` trigger type (see below),
 * `proxy`: an object with a single key, `location`, containing a pair of floating values, representing the geo localized coordinates, when available,
 * `reason`: a string describing why a decision was reached,
 * `profiling`: for now, an empty object,
 * `biometrics`: for now, an empty object.

## Security configuration object

This is an object representing the security configuration when the request was matched:

 * `revision`: string, the revision, from the manifest file,
 * `acl_active`: boolean, true if ACL is enabled,
 * `cf_active`: boolean, true if content filters are enabled,
 * `cf_rules`: amount of content filter rules that were matched against the request,
 * `rate_limit_rules`: amount of rate limit rules,
 * `global_filters_active`: amount of global filters.

## Trigger lists

The fields named `TYPE_triggers` are lists of objects, representing the filter elements that were triggered.
Each of these objects contain information about the part of the request that triggered, as well as information
specific to the type of trigger.

The `active` key is also always present, and is `false` for decisions that did trigger an actual response
(monitor decisions), and `true` otherwise.

### Location data

The following entries are all optional:

 * `section`, can be `attributes`, `ip`, `uri`, `referer_path`, `path`, `body`, `headers` or `body`;
 * `part`, indicate a "path part". Path parts are elements separated by slashes;
 * `name`, name of the argument, header or cookie that triggered the response;
 * `value`, actual value that triggered the response.

### Trigger specific entries

The following triggers are defined:

#### ACL triggers

There usual trigger has the following keys:

  * `tags`, the list of tags that matched the ACL column,
  * `stage`, the ACL column.

Another possibility is:

  * `type`, can be `phase1` or `phase2`, representing the stage where the grasshopper plug-in failed,
  * `details`, an optional entry describing the issue.

#### Rate limit triggers

There is one kind of trigger, with the following keys:

  * `id`, representing the rate limit id,
  * `name`, representing the rate limit name,
  * `threshold`, a number representing the limit threshold,
  * `counter`, representing the actual number of requests (will always be equal to `threshold` + 1).

#### Flow control triggers

There is one kind of trigger, with the following keys:

  * `id`, representing the flow control id,
  * `name`, representing the flow control name.

#### Global filter triggers

There is one kind of trigger, with the following keys:

  * `id`, representing the global filter entry id,
  * `name`, representing the global filter entry name.

#### Content filter triggers

There are several entries, that can be distinguished with the `type` entry:

When `type` is `signature`, it represents a content filter match, with the following entries:

  * `ruleid`, the id of the matching rule,
  * `risk_level`, the risk level of the matching rule.

When `type` is `body_missing`, it means that the body was expected, but was missing.

When `type` is `body_malformed`, it means that the body was expected to be of a specific type (such as Json),
but was malformed. There is no extra information.

When `type` is `body_too_deep`, it means that the body was parsed as a recursive data format (such as Json),
but was deeper than expected. The following entries are present:

  * `expected`, the maximum depth, as defined by the security policy,
  * `actual`, the actual depth where the processing stopped. This is usually `expected` + 1.

When `type` is `sqli`, it represents a *libinjection* SQLI match, with the `fp` entry that holds details.

When `type` is `xss`, it represents a *libinjection* XSS match, with no additional data.

When `type` is `restricted`, it means that some part of the request was marked as restricted and did not
match the corresponding regular expression.

When `type` is `too_many_entries`, it means that there were too many entries in a section, with the following entries:

   * `expected`, maximum amount of entries, as described in the security policy,
   * `actual`, actual amount of entries.

When `type` is `entry_too_large`, it means that a section entry was too large, with the following entries:

   * `expected`, maximum size of entries, as described in the security policy,
   * `actual`, actual size of the offending entry.

# Sample log

```json
{
  "acl_triggers": [],
  "arguments": {
    "v": "xp_cmdshell"
  },
  "authority": "www.example.com",
  "biometrics": {},
  "content_filter_triggers": [
    {
      "active": true,
      "name": "v",
      "risk_level": 5,
      "ruleid": "100016",
      "section": "uri",
      "type": "signature",
      "value": "xp_cmdshell"
    }
  ],
  "cookies": {},
  "flow_control_triggers": [],
  "global_filter_triggers": [],
  "headers": {
    "accept": "*/*",
    "test": "stsd",
    "user-agent": "curl/7.68.0",
    "x-envoy-external-address": "172.24.0.1",
    "x-forwarded-for": "199.0.0.1,172.24.0.1",
    "x-forwarded-proto": "http",
    "x-request-id": "2b486002-5f3a-49cf-85fb-7dc36c7408c7"
  },
  "ip": "199.0.0.1",
  "logs": [
    "D 1µs Inspection init",
    "D 291µs Inspection starts (grasshopper active: true)",
    "D 325µs CFGLOAD logs start",
    "D 3485219µs Loaded profile __default__ with 188 rules",
    "D 340µs CFGLOAD logs end",
    "D 349µs Selected hostmap default entry",
    "D 356µs Selected hostmap entry default",
    "D 398µs map_request starts",
    "D 477µs headers mapped",
    "D 714µs geoip computed",
    "D 789µs uri parsed",
    "D 790µs body parsed",
    "D 790µs args mapped",
    "D 1197µs request tagged",
    "D 1249µs challenge phase2 ignored",
    "D 1257µs flow checks done",
    "D 1259µs no limits to check",
    "D 1263µs limit checks done (0 limits)",
    "D 1378µs ACL result: Match { bot: None, human: None }",
    "D 1717µs matching content filter signatures: true",
    "D 1736µs signature matched [0..11] ContentFilterRule { id: \"100016\", operand: \"xp_(makecab|cmdshell|execresultset|regaddmultistring|regread|enumdsn|availablemedia|regdeletekey|loginconfig|regremovemultistring|regwrite|regdeletevalue|dirtree|regenumkeys|filelist|terminate|servicecontrol|ntsec_enumdomains|terminate_process|ntsec|regenumvalues|cmdshell)\", risk: 5, category: \"sqli\", subcategory: \"built-in function invocation\", tags: {} }",
    "D 1889µs Content Filter checks done"
  ],
  "method": "GET",
  "path": "/api/ds/",
  "path_parts": {
    "part1": "api",
    "part2": "ds",
    "part2:decoded": "v",
    "path": "/api/ds/"
  },
  "processing_stage": 6,
  "profiling": {},
  "proxy": {
    "location": [
      37.751,
      -97.822
    ]
  },
  "rate_limit_triggers": [],
  "reason": "blocking - content filter 100016[lvl5] - [URI argument v=xp_cmdshell]",
  "request_id": null,
  "response_code": 403,
  "security_config": {
    "acl_active": true,
    "cf_active": true,
    "cf_rules": 188,
    "global_filters_active": 1,
    "rate_limit_rules": 0,
    "revision": "0dc9b177e5858d0a26b8cb408fe1fd357cb84d65"
  },
  "tags": [
    "aclid:--default--",
    "cf-rule-subcategory:built-in-function-invocation",
    "cf-rule-category:sqli",
    "api",
    "geo-asn:1239",
    "geo-continent-name:north-america",
    "geo-org:sprintlink",
    "geo-city:nil",
    "aclname:default-acl",
    "ip:199-0-0-1",
    "cf-rule-risk:5",
    "cf-rule-id:100016",
    "securitypolicy:default-entry",
    "host:www-example-com",
    "args:1",
    "geo-region:nil",
    "contentfiltername:default-contentfilter",
    "contentfilterid:--default--",
    "headers:7",
    "bot",
    "all",
    "cookies:0",
    "geo-country:united-states",
    "geo-subregion:nil",
    "securitypolicy-entry:default",
    "geo-continent-code:na"
  ],
  "timestamp": "2022-07-15T14:35:53.757544186Z",
  "trigger_counters": {
    "acl": 0,
    "acl_active": 0,
    "content_filters": 1,
    "content_filters_active": 1,
    "flow_control": 0,
    "flow_control_active": 0,
    "global_filters": 0,
    "global_filters_active": 0,
    "rate_limit": 0,
    "rate_limit_active": 0
  },
  "uri": "/api/ds/?v=xp_cmdshell"
}
```