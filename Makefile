.SILENT:

IMAGE := fizzbuzz

build-development:
	docker build --target development -t $(IMAGE):development .

build-release:
	docker build --target production -t $(IMAGE):release .

build-migration:
	docker build --target migration -t $(IMAGE):migration .

gen:
	cargo run --bin doc | jq '. * input' - api/openapi.template.json > api/openapi.json
	cargo doc --no-deps
