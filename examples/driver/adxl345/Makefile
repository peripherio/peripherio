
driver.so: adxl345.c peripherio.gen.h
	$(CC) adxl345.c -shared -o driver.so

peripherio.gen.h: driver.yml
	../../../target/debug/codegen
