#nukes the data dir, .init.sh will run again on docker up
clean:
	rm -rf ./data
	mkdir ./data


migrate:
	./scripts/migrate.sh