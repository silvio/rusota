FROM scratch
ARG VERSION=debug

label org.opencontainers.image.source="https://github.com/silvio/rusota"
label org.opencontainers.image.descriptioni="Docker image for a rust based OTA Update server for LineageOS"
label org.opencontainers.image.licenses="MIT"

copy target/x86_64-unknown-linux-musl/${VERSION}/rusota /rusota

CMD [ "/rusota" ]
