# syntax=docker/dockerfile:experimental
# Build the service
FROM rust:1.84.0 as build

WORKDIR /build
COPY Cargo.toml .
COPY Cargo.lock .

## To cache build dependencies in docker stack.
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo build --release


# Run the service

FROM gcr.io/distroless/cc-debian12

WORKDIR /app
COPY --from=build /build/target/release/tailwag_application_template ./start_service
COPY ./static .
# RUN apt-cache search ssl
ENTRYPOINT ["./start_service"]
#ENTRYPOINT ["target/release/rest_service_template"]
