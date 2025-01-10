PROJECTS := $(shell find . -type f -name "Cargo.toml" | xargs -n 1 dirname)

.PHONY: install clean $(PROJECTS)

install: $(PROJECTS)
	@echo "All projects installed!"

$(PROJECTS):
	@echo "Installing $@..."
	@(cd $@ && cargo clean && cargo install --path .)
	@echo "Installed $@"

clean:
	@for project in $(PROJECTS); do \
		echo "Cleaning $$project..."; \
		(cd $$project && cargo clean); \
	done
	@echo "All projects cleaned!"
