CONTRACTS=storage application

WASM_TARGETS=$(patsubst %,../contract/%/contract.wasm.gz,$(CONTRACTS))
SCHEMA_TARGETS=$(patsubst %,../contract/%/schema/index.d.ts,$(CONTRACTS))
TARGETS=$(WASM_TARGETS) $(SCHEMA_TARGETS)

ENVS=mainnet pulsar local team
ACTIONS=balance deploy test info
ENVS_ACTIONS=$(foreach e,$(ENVS), $(foreach a,$(ACTIONS),$(e)-$(a)))

help:
	@echo "make build"
	@echo "make <env>-<action>"
	@echo "  env: $(ENVS)"
	@echo "  action: $(ACTIONS)"

$(ENVS_ACTIONS): build
	@e=$$(echo -n "$@"|cut -d - -f 1); \
	a=$$(echo -n "$@"|cut -d - -f 2); \
	echo npx ts-node --esm $$a.ts config-$$e.json; \
	npx ts-node --esm $$a.ts config-$$e.json

build: $(TARGETS) $(SCHEMA_TARGETS)

clean:
	for c in $(CONTRACTS); do $(MAKE) -C ../contract/$$c clean; done

$(WASM_TARGETS):
	$(MAKE) -C $(@D) clean
	$(MAKE) -C $(@D) build-mainnet
	#$(MAKE) -C $(@D) build-mainnet-reproducible

$(SCHEMA_TARGETS):
	echo $@
	$(MAKE) -C $$(dirname $(@D)) schema

.PHONY: deploy build clean
