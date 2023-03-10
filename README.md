# structured-ripgrep (`srg`)

This tool lets you filter structured data using ripgrep. Supported formats:

* JSON - output objects where a specific key matches a pattern
* CSV - outputs rows where a specific column matches a pattern
* tar - outputs a new tar file with only files that match a pattern
* length-prefixed text - given an input of `{key}\t{length}\t{text}`, output `key`s when `text` matches a pattern

### Why?

Ripgrep is powerful, but sometimes you need to use the power of ripgrep over only a certain field, column or entry 
within a stream. Imagine you are downloading a very large tar file from the internet but only need files that match 
a specific pattern - you can pipe the download into `srg`, filter out files you don't want and then pipe the contents 
into `tar` for extraction or other processing.

### Examples

```shell
$ curl "https://random-data-api.com/api/v2/banks?size=100" \
    | jq -c '.[]' \
    | srg 'LIMITED$' json 'bank_name'
{"id":4918,"uid":"a4704ac1-52d1-474d-a097-219b6db62bfa","account_number":"6812368096","iban":"GB06YUYD93346224006762",
"bank_name":"OTKRITIE SECURITIES LIMITED","routing_number":"708856653","swift_bic":"AANLGB21"}
```

```shell
$ cat tar_file.tar \
    | srg 'some.*pattern$' tar > filtered_tar.tar
```

```shell
$ curl https://perso.telecom-paristech.fr/eagan/class/igr204/data/factbook.csv \
    | srg '^Republic.*' 1 --delimiter=';'
Central African Republic,...
Czech Republic,...
Dominican Republic,...
```
