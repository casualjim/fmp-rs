FROM ghcr.io/casualjim/bare:libcxx-ssl

ENV RUST_LOG=info

COPY dist/fmp /usr/bin/app
USER appuser
ENTRYPOINT ["/usr/bin/app"]
