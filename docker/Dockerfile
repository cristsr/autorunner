FROM ubuntu:focal as build

ARG RUNNER_VERSION
ARG TARGET_ARCH
ARG NODE_VERSION

ENV RUNNER_URL=https://github.com/actions/runner/releases/download/v${RUNNER_VERSION}/actions-runner-linux-${TARGET_ARCH}-${RUNNER_VERSION}.tar.gz
ENV NODE_URL=https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-${TARGET_ARCH}.tar.gz

WORKDIR /actions-runner

RUN apt update -y && apt install curl unzip -y
RUN apt clean && apt autoremove -y

WORKDIR /actions-runner

RUN echo "Downloading runner from ${RUNNER_URL}"

RUN mkdir -p runner nodejs

# download actions runner
RUN curl -f -L -o runner.tar.gz ${RUNNER_URL} \
    && tar xzf ./runner.tar.gz -C runner \
    && rm runner.tar.gz

# download nodejs
RUN curl -f -L -o nodejs.tar.gz ${NODE_URL} \
    && tar xzf ./nodejs.tar.gz --strip-components=1 -C nodejs\
    && rm nodejs.tar.gz

FROM mcr.microsoft.com/dotnet/runtime-deps:6.0-jammy

ENV DEBIAN_FRONTEND=noninteractive
ENV RUNNER_MANUALLY_TRAP_SIG=1
ENV ACTIONS_RUNNER_PRINT_LOG_TO_STDOUT=1
ENV ImageOS=ubuntu22

## Config user
RUN adduser --disabled-password --gecos "" --uid 1001 runner \
    && groupadd docker --gid 123 \
    && usermod -aG sudo runner \
    && usermod -aG docker runner \
    && echo "%sudo   ALL=(ALL:ALL) NOPASSWD:ALL" > /etc/sudoers \
    && echo "Defaults env_keep += \"DEBIAN_FRONTEND\"" >> /etc/sudoers

WORKDIR /home/runner

COPY --chown=runner:docker --from=build /actions-runner/runner .
COPY --from=build /actions-runner/nodejs/ /usr/local/lib/nodejs/
COPY --from=docker:dind /usr/local/bin/docker /usr/local/bin/
COPY ../scripts/entry.sh .

ENV PATH=/usr/local/lib/nodejs/bin:$PATH

ENTRYPOINT ["./entry.sh"]

USER runner


# docker build -t runner -f .\images\Dockerfile .  --build-arg RUNNER_VERSION=2.317.0 --build-arg TARGET_ARCH=x64 --build-arg NODE_VERSION=20.14.0

# docker run -it -e ORG_URL=https://github.com -e REG_TOKEN=token runner -e RUNNER_NAME=runner
