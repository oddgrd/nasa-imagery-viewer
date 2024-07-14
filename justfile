# start development

# start the site in dev mode
run-dev:
  cd site && trunk serve --open

# build for deployment
build-release:
  cd site && trunk clean && trunk build --release

# run locally with shuttle
shuttle-run:
  cargo shuttle run

# deploy to shuttle
shuttle-deploy:
  cargo shuttle project restart && cargo shuttle deploy