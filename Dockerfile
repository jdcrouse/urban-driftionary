FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/ud-api
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates
COPY --from=builder /usr/local/cargo/bin/ud-api /usr/local/bin/ud-api
CMD ["ud-api"]

EXPOSE 8000