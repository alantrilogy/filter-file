# Build base
FROM rust:latest as build 

# Create a new binary project
RUN USER=root cargo new --bin filter_file
WORKDIR /filter_file

# Copy over manifests
COPY ./Cargo.lock ./Cargo.lock 
COPY ./Cargo.toml ./Cargo.toml

# Build and cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source 
COPY ./src ./src
COPY ./sample ./sample

# Build binary for release
RUN rm ./target/release/deps/filter_file*
RUN cargo build --release

# Release base
FROM debian:buster-slim

# Copy build artefact
COPY --from=build /filter_file/target/release/filter_file .
COPY --from=build /filter_file/sample/*.* .

ENV DEBUG=error
ENV FILE="code_file.rs"

# Run command
CMD ./filter_file ${FILE} --debug ${DEBUG}