FROM scratch
ARG VERSION=debug

copy target/x86_64-unknown-linux-musl/${VERSION}/rusota /rusota

CMD [ "/rusota" ]
