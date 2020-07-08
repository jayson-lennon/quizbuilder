run bin:
    cd {{bin}} && cargo +nightly run

build bin:
    cd {{bin}} && cargo +nightly build

check:
    cargo +nightly check