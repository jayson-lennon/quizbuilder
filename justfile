run bin:
    #!/bin/bash
    if [ {{bin}} == "frontend" ]; then
        cd frontend && npm start
    else
        cd backend/{{bin}} && cargo +nightly run
    fi

build bin:
    cd backend/{{bin}} && cargo +nightly build

check:
    cd backend && cargo +nightly check
