define ReadEnvVar
$(shell jq -r '.${1}' ./env.json)
endef

test: 
	cargo test -- --nocapture

local:
	HOST=$(call ReadEnvVar,HOST) \
	PORT=$(call ReadEnvVar,PORT) \
	RUST_LOG=$(call ReadEnvVar,RUST_LOG) \
	PARALLEL_FILES=$(call ReadEnvVar,PARALLEL_FILES) \
	PAYLOAD_MAX_SIZE=$(call ReadEnvVar,PAYLOAD_MAX_SIZE) \
	IS_TEST=$(call ReadEnvVar,IS_TEST) \
	RUST_BACKTRACE=1 \
	cargo watch -x 'run'

docker:
	docker run -it cloudfoundry/cflinuxfs4 bash
	
push:
	git add .
	git commit -m "$(shell read -p "Enter Commit Message: " enter ; echo $${enter})"
	git push -u origin main

