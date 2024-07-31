ARG builder
ARG img

FROM $builder as builder
WORKDIR /build
COPY ./ /build
RUN cargo build --release

FROM $img
COPY --from=builder /build/target/release/cronized /chronized