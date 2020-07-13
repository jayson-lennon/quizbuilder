run bin:
    #!/bin/bash
    if [ {{bin}} == "frontend" ]; then
        cd frontend && npm start
    else
        cd backend/{{bin}} && cargo +nightly run
    fi

build bin:
    #!/bin/bash
    if [ {{bin}} == "frontend" ]; then
        cd frontend && npm run build
    else
        cd backend/{{bin}} && cargo +nightly build
    fi

check:
    cd backend && cargo +nightly check

copy-frontend:
    rm -Rf backend/quizmgmtd/static
    rsync -ahxv frontend/dist/frontend/ backend/quizmgmtd/static
    mv backend/quizmgmtd/static/index.html backend/quizmgmtd/static/new_quiz.html

sample-quizmgmt: (build "frontend") copy-frontend
    just run quizmgmtd