ARG PLATFORM

FROM --platform=${PLATFORM} rust:1.84.0-slim-bookworm AS build

RUN apt update && apt install -y libssl-dev

WORKDIR /app

COPY ./hello_world_compile_test .

RUN cargo build --release

FROM --platform=${PLATFORM} rust:1.84.0-slim-bookworm AS prod

COPY --from=build /app/target/release/hello_world_compile_test /opt/hello_world_compile_test/

CMD ["/opt/hello_world_compile_test/hello_world_compile_test"]