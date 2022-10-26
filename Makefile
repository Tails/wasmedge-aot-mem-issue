# build after changing the base image
build:
	docker-compose build wasmedge

# run when the base image hasnt changed
run:
	docker-compose up \
		--no-recreate \
		wasmedge

run-normal:
	docker-compose up \
		wasmedge

# run after updating the base image
run-new:
	docker-compose up \
		--force-recreate \
		wasmedge