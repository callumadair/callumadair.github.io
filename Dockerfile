FROM alpine:latest

RUN apk add docker docker-compose just openrc zellij
RUN rc-update add docker boot
COPY . /workspace
WORKDIR /workspace

LABEL authors="cal"

CMD ["just", "compose"]