FROM golang:1.8-alpine3.6

COPY . /go/src/taste/
RUN go get github.com/gocql/gocql

WORKDIR /go/src/taste/
CMD go test -v ./...
