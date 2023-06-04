FROM rust:1.69 as build
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# create a new empty shell project
RUN USER=root cargo new --bin count-loc-action
WORKDIR /count-loc-action

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/count_loc_action*
RUN cargo build --release


FROM gcr.io/distroless/cc AS runtime
COPY --from=build /count-loc-action/target/release/count-loc-action .
ENTRYPOINT ["/count-loc-action"]