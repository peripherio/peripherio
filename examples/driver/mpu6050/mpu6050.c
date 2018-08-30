#include <string.h>
#include <stdlib.h>
#include <glob.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <linux/i2c-dev.h>

#include "rami.gen.h"

get_gyro_returns* get_gyro(get_gyro_args* args, Config* conf) {
  /* Your Implementation! */
}

get_accel_returns* get_accel(get_accel_args* args, Config* conf) {
  /* Your Implementation! */
}

get_temperature_returns* get_temperature(get_temperature_args* args, Config* conf) {
  /* Your Implementation! */
}

void init() {
}

Config** detect(Config* conf, size_t* size) {
  Config* results[100];
  size_t results_count = 0;

  glob_t globbuf;
  glob("/dev/i2c-*", 0, NULL, &globbuf);
  for (int i = 0; i < globbuf.gl_pathc; i++) {
    int fd = open(globbuf.gl_pathv[i], O_RDWR);
    if (fd < 0) {
      return NULL;
    }
    for (u_int8_t addr = 0x03; addr < 0x78; addr++) {
      if (ioctl(fd, I2C_SLAVE, addr) < 0) {
        continue;
      }

      /* Read who_am_i */
      u_int8_t reg = 0x75;
      if ((write(fd, &reg, 1)) != 1) {
        return NULL;
      }
      u_int8_t dat;
      if (read(fd, &dat, 1) != 1) {
        return NULL;
      }
      if(dat != 0b01101000) {
        continue;
      }

      /* Device found with bus/addr */
      Config* new_config = malloc(sizeof(Config));
      memcpy(new_config, conf, sizeof(Config));

      new_config->if_i2c_speed = 100l;
      new_config->if_i2c_busnum = i; // FIXME: Maybe have to parse the glob result
      new_config->if_i2c_address = addr;

      results[results_count++] = new_config;
      results_count++;
    }
  }
  globfree(&globbuf);

  *size = results_count;
  Config** res = malloc(sizeof(Config*)*results_count);
  for(size_t i = 0; i < results_count; i++) {
    res[i] = malloc(sizeof(Config));
    memcpy(res[i], results[i], sizeof(Config));
  }
  return res;
}

