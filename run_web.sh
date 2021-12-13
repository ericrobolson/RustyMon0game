# This script runs the web client
# https://yarnpkg.com/package/http-server

# Build pkg if it doesn't exist
[[ ! -d web-client/pkg ]] && ./build_web.sh 

cd web-client
live-server --watch=.
