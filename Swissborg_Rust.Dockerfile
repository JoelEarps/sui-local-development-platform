FROM docker.sharedborg.com/rust-runtime:20240410-2619
ARG BIN_NAME
COPY ./target/release/${BIN_NAME} /usr/local/bin/${BIN_NAME}
