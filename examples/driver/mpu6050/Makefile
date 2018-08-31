
driver.so: driver.c peripherio.gen.h
	$(CC) mpu6050.c -shared -o driver.so

peripherio.gen.h: driver.yml
	../../target/debug/codegen
