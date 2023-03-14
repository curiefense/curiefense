# List of non user tags

Non-user tags are the tags that the user did not configure. They are hard-coded, and can't be avoided.

## Systematic

 * `all`
 * `headers:n`: annotated with the amount of headers
 * `cookies:n`: annotated with the amount of cookies
 * `args:n`: annotated with the amount of arguments
 * `host:xxx`
 * `ip:xxx`
 * `geo-continent-name:xxx`
 * `geo-continent-code:xxx`
 * `geo-city:xxx`
 * `geo-org:xxx`
 * `geo-country:xxx`
 * `geo-region:xxx`
 * `geo-subregion:xxx`
 * `geo-asn:xxx`
 * `network:xxx`

## Optional

Most of these are dependent on the use of the IPinfo database:

 * `geo-anon`
 * `geo-sat`
 * `geo-vpn`
 * `geo-tor`
 * `geo-relay`
 * `geo-hosting`
 * `geo-privacy-service`
 * `geo-mobile`

## Humanity testing

 * `human` or `bot`

Depending on the human verification plug-in and configuration, once of these can appear:

 * `precision-l1`
 * `precision-l3`
 * `precision-l4`
 * `mobile-sdk:emulator`

## Flow control

For each *matching* flow control:

  * `fc-id:xxx`
  * `fc-name:xxx`

## Limits

For each *matching* limit:

  * `limit-id:xxx`
  * `limit-name:xxx`

## Content filter

For each *matching* content filter rule:

 * `cf-rule-id:xxx`
 * `cf-rule-category:xxx`
 * `cf-rule-subcategory:xxx`
 * `cf-rule-risk:xxx`