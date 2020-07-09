run bin:
    cd backend/{{bin}} && cargo +nightly run

build bin:
    cd backend/{{bin}} && cargo +nightly build

check:
    cd backend && cargo +nightly check