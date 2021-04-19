FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin k-monitor
WORKDIR ./k-monitor
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/k_monitor*
RUN cargo build --release


FROM alpine:latest

ARG APP=/usr/src/app

EXPOSE 8080
EXPOSE 8082

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/k-monitor/target/x86_64-unknown-linux-musl/release/k-monitor ${APP}/k-monitor
RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}
CMD ["./k-monitor"]