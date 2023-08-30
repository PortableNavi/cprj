target_name := @[name]
project_version := @[version]
src_files := main.c

$(target_name): $(src_files)
	gcc $^ -o $@

.DEFAULT_GOAL: $(target_name))

.PHONY: run
run:
	./$(target_name)



