#!/bin/bash

nohup redis-server &
cargo t
