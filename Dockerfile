ARG CROSS_PLATFORM
ARG CROSS_VERSION
FROM rustembedded/cross:$CROSS_PLATFORM-$CROSS_VERSION

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes libasound2-dev:armhf

ENV PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig
