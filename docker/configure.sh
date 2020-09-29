#!/bin/bash

if [[ -n "${LOSS}" ]]; then
  tc qdisc add dev eth0 root handle 1: netem loss random $LOSS
else
  echo "LOSS not set"
fi

if [[ -n "${RATE}" ]]; then
  if [[ -n "${BURST}" ]]; then
    if [[ -n "${LATENCY}" ]]; then
      if [[ -n "${LOSS}" ]]; then
        tc qdisc add dev eth0 parent 1:1 handle 10: tbf rate $RATE latency $LATENCY burst $BURST
      else
        tc qdisc add dev eth0 root tbf rate $RATE latency $LATENCY burst $BURST
      fi
    else
      echo "LATENCY not set"
    fi
  else
    echo "BURST not set"
  fi
else
  echo "RATE not set"
fi