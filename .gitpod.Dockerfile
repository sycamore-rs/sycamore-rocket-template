FROM gitpod/workspace-full

# Install trunk
RUN bash -cl "wget -qO- https://github.com/thedodd/trunk/releases/download/v0.13.1/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-"
RUN bash -cl "sudo mv ./trunk /usr/bin/"

# Install cargo-watch
RUN bash -cl "wget https://github.com/watchexec/cargo-watch/releases/download/v7.7.2/cargo-watch-v7.7.2-x86_64-unknown-linux-gnu.tar.xz \
    && tar -xf cargo-watch-v7.7.2-x86_64-unknown-linux-gnu.tar.xz"
RUN bash -cl "sudo mv ./cargo-watch-v7.7.2-x86_64-unknown-linux-gnu/cargo-watch ~/.cargo/bin"

# Install wasm32-unknown-unknown target
RUN rustup target add wasm32-unknown-unknown
