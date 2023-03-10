# structured-ripgrep (`srg`)

This tool lets you filter structured data using ripgrep. Supported formats:

* JSON
* CSV
* tar
* length-delimitated text

### Example

```shell
$ curl "https://random-data-api.com/api/v2/banks?size=100" | jq -c '.[]' | srg 'LIMITED$' json 'bank_name'
{"id":4918,"uid":"a4704ac1-52d1-474d-a097-219b6db62bfa","account_number":"6812368096","iban":"GB06YUYD93346224006762",
"bank_name":"OTKRITIE SECURITIES LIMITED","routing_number":"708856653","swift_bic":"AANLGB21"}
```
