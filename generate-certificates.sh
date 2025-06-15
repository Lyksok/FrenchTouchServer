#!/bin/bash

openssl req -x509 -newkey rsa:4096 -keyout ssl/server.key -out ssl/server.crt -days 365 -nodes -config openssl.cnf -extensions v3_req
