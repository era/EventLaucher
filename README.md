# EventLaucher
:rocket: Rust software that consumes a queue and lauches any script to handle the data. Everything customisable by a config file.


## Draft idea

config.yaml

```yaml

my_cool_queue:
  -
    rule: ".event.type eq 'error'"
    exec: "python3 slack_message.py ${event}"
  -
    rule: ".event.type eq 'success'"
    exec: "python3 send_email_boss.py ${event}"

another_queue:
  -
    rule: ".event.type eq 'error'"
    exec: "python3 slack_message.py ${event}"
  -
    rule: ".event.type eq 'success'"
    exec: "python3 send_email_boss.py ${event}"

```

Special variables are:

- `${event}`: the exactly event from the queue.
- `${received_at}`: timestamp

The idea of the rules is to support queues which messages are JSON. The syntax is similar to `jq`

- Parse rules
  - {event} op {target}
  	- op can be eq, !eq, >, <
	- target is either a string or a number
	- \$(\.{1}[a-z0-9]{1,}){0,} (eq|!eq|>|<) ([0-9a-z ]){1,}
	- with groups: (\$(\.{1}[a-z0-9]{1,}){0,}) (eq|!eq|>|<) ([0-9a-z ]{1,})
		- group 1=json, group2=garbage, group3=op, group4=target
- Consume Queues
- Exec command or log error


## Mac
To build you need: 
`export JQ_LIB_DIR=/usr/local/bin/jq`