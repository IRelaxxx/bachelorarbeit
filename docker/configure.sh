#!/bin/bash

if [-z "$LOSS_PERCENT"]; then
  tc qdisc add dev eth0 root netem loss random $LOSS_PERCENT
fi

if [-z "$RATE"]; then
  tc qdisc add dev eth0 root tbf rate $RATE burst $BURST
fi