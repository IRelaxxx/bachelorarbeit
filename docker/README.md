This dockerfile contains a layer to setup network emulation using tc netem and tbf for a source container.
The configuration is supplied via environment variables

| Variable Name | Description                                    |
| ------------- | ---------------------------------------------- |
| LOSS_PERCENT  | Determines how many Network packages are lost  |
| RATE          | max bandwidth                                  |
| BURST         | Bucket size                                    |
| LATENCY       | Maximum amount of time a packet can be held up |

The resulting image must run in a privileged container.

For further information see https://linux.die.net/man/8/tc-tbf
