# nginx-accesslog-count

Count frequencies of columns in nginx access logs.

## Installation

TODO

## Usage

```
nginx-accesslog-count [OPTIONS] --column <COLUMN> <FILES>...

Arguments:
<FILES>...  Whitespace separated list of log files to read

Options:
-c, --column <COLUMN>        Column to count [possible values: address, user, time, request, status, bytes_sent, referer, user_agent]
-o, --order <ORDER>          Order to sort [default: desc] [possible values: asc, desc]
-l, --limit <LIMIT>          Number of lines to show. 0 will show all lines [default: 0]
-q, --quiet                  Suppress all output except result list
-n, --no-count               Show only column value without count
-e, --exclude <EXCLUDE>      Exclude lines where column matches regex pattern. [default: ]
-w, --whitelist <WHITELIST>  Only include lines where column matches regex pattern. [default: ]
-h, --help                   Print help
-V, --version                Print version
```

## Example

```
$ nginx-accesslog-count -c request -l 10 /var/log/nginx/access*

Processing 56.21 MB of logs in 15 files.
Finished processing in 621ms 79us 200ns.
Showing top 10 results.

      1926 - GET / HTTP/1.1
       448 - GET /favicon.ico HTTP/1.1
       414 - GET /.env HTTP/1.1
       304 - POST / HTTP/1.1
       205 - GET / HTTP/1.0
       195 - GET /logo192.png HTTP/1.1
       180 - GET /static/js/main.cc1d4eee.js HTTP/1.1
       150 - GET /logo512.png HTTP/1.1
       148 - GET /about HTTP/1.1
       145 - GET /static/css/main.111e0214.css HTTP/1.
```