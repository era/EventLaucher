# EventLaucher
:rocket: Rust software that consumes a rabbitmq queue and launches any script to handle the data. Everything is customisable by a yaml config file.

It was made for my personal usage (and for me to learn Rust). Before using, read the code. If you want to improve anything send a PR, or just open an issue :).

I use it together with https://github.com/era/malleable-checker to send me a message when checkers fail.

## How it works

You need a yaml configuration file like:

```yaml

my_cool_queue:
  -
    rule: ".event.type eq error"
    exec: "python3 slack_message.py ${event}"
  -
    rule: ".event.type eq success"
    exec: "python3 send_email_boss.py ${event}"

another_queue:
  -
    rule: ".event.type eq error"
    exec: "python3 slack_message.py ${event}"
  -
    rule: ".event.type eq success"
    exec: "python3 send_email_boss.py ${event}"

```

Rule works like: `{json_path} op {value}`, where `json_path` follows `jq` syntax, `op`can be `eq`, `!eq`, `>` or `<`. and `value` can be any string or numeric value.

If rule results in true, the command defined at `exec` is executed. If you want to pass the json event for the command you can by using `${event}`.

To execute just cargo build and:

`target/event_launcher example.yaml amqp://localhost`

Where example.yaml is the configuration yaml and the second argument is the RabbitMQ URL to connect. If you need to pass a password on the URL itself, you will need to modify the code to avoid passing it as argument (since your password will be at `history`).

## Mac
To build you need `jq` installed (you probably have) and setup `JQ_LIB_DIR`: 
`export JQ_LIB_DIR=/usr/local/bin/jq`