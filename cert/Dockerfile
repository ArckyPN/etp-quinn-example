FROM ubuntu:latest

WORKDIR /build

RUN apt-get update && \
    apt install -y libnss3-tools golang openssl xxd ca-certificates

COPY go.mod go.sum ./
RUN go mod download

COPY . .

VOLUME /cert

CMD ./get-cert.sh && cp localhost* /cert