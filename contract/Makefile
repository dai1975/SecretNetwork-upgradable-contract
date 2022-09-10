SUBDIRS=storage application

build:
	@for d in $(SUBDIRS); do $(MAKE) -C $$d $@; done

clean:
	@for d in $(SUBDIRS); do $(MAKE) -C $$d $@; done
	rm -f target

.PHONY: clean build
