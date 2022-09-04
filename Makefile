include .env.local

run:
	@DATABASE_URL=$(DATABASE_URL) cargo run

proxy:
	@flyctl proxy 5432 -a $(DB_APPNAME)

deploy:
	@flyctl deploy