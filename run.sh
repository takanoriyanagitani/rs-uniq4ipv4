#!/bin/bash

input(){
	printf '\x7f\0\0\2'
	printf '\x7f\0\0\3'
	printf '\x7f\0\0\5'
	printf '\x7f\0\0\7'
	printf '\x7f\0\0\3'
}

export ENV_RAW_MODE=false
input | ./rs-uniq4ipv4

export ENV_RAW_MODE=true
input | ./rs-uniq4ipv4 | xxd
