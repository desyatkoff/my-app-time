#!/usr/bin/env bash

if [[ ! -d ".git" && ! -f "install.sh" ]]; then
    if [[ ! -d "my-app-time" ]]; then
        git clone --depth 1 https://github.com/desyatkoff/my-app-time.git
    fi

    cd "my-app-time"
fi

cargo clean || true

[ -f /usr/bin/mat ] && sudo rm -vf /usr/bin/mat || true

cargo build --release

sudo cp -v \
    "./target/release/my-app-time" \
    "/usr/bin/mat"
