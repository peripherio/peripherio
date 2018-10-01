#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <glob.h>
#include <unistd.h>
#include <stdint.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <linux/i2c-dev.h>

#include "peripherio.gen.h"

static const float ACCEL_SCALE = 32.0 / 8192.0;

static const uint8_t DATA_FORMAT = 0x31;
static const uint8_t POWER_TCL = 0x2D;

static const uint8_t DATAX0 = 0x32;
static const uint8_t DATAY0 = 0x34;
static const uint8_t DATAZ0 = 0x36;

int i2c_read(int fd, uint8_t reg, uint8_t* dat) {
  if ((write(fd, &reg, 1)) != 1) {
    return -1;
  }

  if (read(fd, dat, 1) != 1) {
    return -1;
  }
  return 0;
}

int i2c_write(int fd, uint8_t reg, uint8_t dat) {
  uint8_t buf[2] = {reg, dat};
  if (write(fd, buf, 2) != 2) {
    return -1;
  }
  return 0;
}

int i2c_read_word(int fd, uint8_t reg, uint16_t* dat) {
  uint8_t high;
  if (i2c_read(fd, reg, &high) != 0) {
    return -1;
  }

  uint8_t low;
  if (i2c_read(fd, reg+1, &low) != 0) {
    return -1;
  }

  uint16_t value = (high << 8) + low;

  *dat = value;
}

int use_config(Config* conf) {
  if (ioctl(conf->adxl345_fd, I2C_SLAVE, conf->if_i2c_address) < 0) {
    return -1;
  }
  if (i2c_write(conf->adxl345_fd, DATA_FORMAT, 0x0A) != 0) {
    return -1;
  }
  if (i2c_write(conf->adxl345_fd, POWER_TCL, 0x08) != 0) {
    return -1;
  }
  return 0;
}

get_accel_returns* get_accel(get_accel_args* args, Config* conf) {
  use_config(conf);
  int fd = conf->adxl345_fd;

  uint16_t x, y, z;
  i2c_read_word(fd, DATAX0, &x);
  i2c_read_word(fd, DATAY0, &y);
  i2c_read_word(fd, DATAZ0, &z);

  get_accel_returns* res = malloc(sizeof(get_accel_returns));
  res->x = x * ACCEL_SCALE;
  res->y = y * ACCEL_SCALE;
  res->z = z * ACCEL_SCALE;

  return res;
}

void init() {
}

Config** detect(Config* conf, size_t* size) {
  Config* results[100];
  size_t results_count = 0;

  glob_t globbuf;
  glob("/dev/i2c-*", 0, NULL, &globbuf);
  for (int i = 0; i < globbuf.gl_pathc; i++) {
    const char* path = globbuf.gl_pathv[i];
    int fd = open(path, O_RDWR);
    if (fd < 0) {
      close(fd);
      return NULL;
    }
    unsigned count_per_fd = 0;
    for (uint8_t addr = 0x03; addr < 0x78; addr++) {
      if (ioctl(fd, I2C_SLAVE, addr) < 0) {
        continue;
      }

      /* Read DEVID */
      uint8_t dat;
      if (i2c_read(fd, 0x00, &dat) != 0) {
        continue;
      }
      if(dat != 0b11100101) {
        continue;
      }

      /* Device found with bus/addr */
      Config* new_config = malloc(sizeof(Config));
      memcpy(new_config, conf, sizeof(Config));

      new_config->if_i2c_speed = (int64_t)100;
      new_config->if_i2c_address = addr;

      /* parse glob result and obtain bus number */
      unsigned ci = 0;
      for (ci = strlen(path); ci > 0; ci--) {
        if (path[ci] == '-') { // 3 in i2c-n
          break;
        }
      }
      long busnum = strtol(path + ci + 1, NULL, 10);
      new_config->if_i2c_busnum = busnum;
      new_config->adxl345_fd = fd;

      results[results_count++] = new_config;
      count_per_fd++;
    }
    if(count_per_fd == 0)
      close(fd);
  }
  globfree(&globbuf);

  *size = results_count;
  Config** res = malloc(sizeof(Config*)*results_count);
  for(size_t i = 0; i < results_count; i++) {
    res[i] = malloc(sizeof(Config));
    *res[i] = *results[i];
  }
  return res;
}

