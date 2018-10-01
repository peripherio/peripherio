
driver.so: mpu6050.c peripherio.gen.h
	$(CC) mpu6050.c -shared -o driver.so

peripherio.gen.h: driver.yml
	PERIPHERIO_DRIVER=$(realpath ../) \
	PERIPHERIO_CATEGORY=$(realpath ../../category) \
	../../../target/debug/codegen
