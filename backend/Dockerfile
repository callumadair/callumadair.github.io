FROM rust:alpine

EXPOSE 8080
# Add alpine dependencies.
RUN apk add curl mold musl-dev perl

# Copy project files over.
COPY . /backend-workspace
WORKDIR /backend-workspace/backend