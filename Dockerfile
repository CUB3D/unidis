FROM rust:latest AS build

WORKDIR /home/code

RUN apt-get update -y && apt-get install -y cmake build-essential git

RUN git clone https://github.com/emscripten-core/emsdk

RUN cd emsdk && ./emsdk install latest && ./emsdk activate latest

RUN rustup target add wasm32-unknown-emscripten

COPY ./web-bridge ./web-bridge
COPY ./libunidis ./libunidis

SHELL ["/bin/bash", "-c"] 

RUN source /home/code/emsdk/emsdk_env.sh && cd ./web-bridge && CXXFLAGS="-fwasm-exceptions" RUSTFLAGS='-C link-arg=-s -C link-arg=EXPORTED_RUNTIME_METHODS=["HEAPU8"] -C link-arg=-s -C link-arg=EXCEPTION_STACK_TRACES=1 -C link-arg=-s -C link-arg=ASSERTIONS=2 -C link-arg=-s -C link-arg=ALLOW_MEMORY_GROWTH=1 -C link-arg=-s -C link-arg=EXCEPTION_DEBUG=1' CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_LINKER=em++ cargo build --release --target=wasm32-unknown-emscripten

RUN cp /home/code/web-bridge/target/wasm32-unknown-emscripten/release/web-bridge.js /home/code/web-bridge/dist/
RUN cp /home/code/web-bridge/target/wasm32-unknown-emscripten/release/web_bridge.wasm /home/code/web-bridge/dist/

# For local testing with build.sh
RUN /bin/bash

FROM nginx:latest

RUN apt-get update -y && apt-get install -y curl
HEALTHCHECK --interval=30s --timeout=3s CMD curl -f http://localhost:80/ || exit 1

COPY --from=build /home/code/web-bridge/dist /usr/share/nginx/html

EXPOSE 80