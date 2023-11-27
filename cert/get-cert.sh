#!/bin/bash

CRT="localhost.pem"
KEY="localhost-key.pem"

go run filippo.io/mkcert -install

go run filippo.io/mkcert -ecdsa -days 10 -cert-file "$CRT" -key-file "$KEY" localhost 127.0.0.1 ::1

openssl x509 -in localhost.pem -outform der | openssl dgst -sha256 -binary | xxd -p -c 256 > localhost.hex