# First time build
```bash
docker-compose build
docker-compose run trvweb npm install
```

# Turning on development environment
```bash
docker-compose up
```

# Tests
## api: unit && integration
```bash
cd api/
cargo test
```

## web: unit
```bash
docker-compose run trvweb bash
npm test
```

## e2e
```bash
cd e2e/
npm test
```
or for a gui
```bash
cd e2e/
npm run cypress:open
```