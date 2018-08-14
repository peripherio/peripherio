#pragma once

#include <string>
#include <cstdlib>
#include <unistd.h>

#include <fcntl.h>
#include <sys/ioctl.h>
#include <linux/i2c-dev.h>

namespace rami::interface {

class i2c {
public:

  template<typename Predicate>
  static std::uint8_t find(std::uint8_t bus, Predicate pred) {
    auto const dev = "/dev/i2c-" + std::to_string(bus);
    int fd;
    if ((fd = ::open(dev.c_str(), O_RDWR)) < 0) {
      throw std::runtime_error("Failed to open the I2C Bus");
    }

    for (std::uint8_t addr = 0x03; addr < 0x78; addr++) {
      if (::ioctl(fd, I2C_SLAVE, addr) < 0) {
        continue;
      }
      try {
        if(pred(i2c(bus, addr)))
          return addr;
      } catch (...) {
        continue;
      }
    }
    return 0;
  }

  i2c(std::uint8_t bus, std::uint8_t address) : bus(bus), address(address) {
    auto const dev = "/dev/i2c-" + std::to_string(bus);
    if ((this->fd = ::open(dev.c_str(), O_RDWR)) < 0) {
      throw std::runtime_error("Failed to open the I2C Bus");
    }

    if (::ioctl(this->fd, I2C_SLAVE, address) < 0) {
      throw std::runtime_error("Unable to get bus access to talk to slave");
    }
  }

  std::uint8_t read(std::uint8_t reg) {
    if ((::write(this->fd, &reg, 1)) != 1) {
      throw std::runtime_error("Error writing to i2c slave");
    }

    std::uint8_t dat;
    if (::read(this->fd, &dat, 1) != 1) {
      throw std::runtime_error("Error reading from i2c slave");
    }
    return dat;
  }

  void write(std::uint8_t reg, std::uint8_t dat) {
    std::uint8_t buf[2] = {reg, dat};
    if ((::write(this->fd, buf, 2)) != 2) {
      throw std::runtime_error("Error writing to i2c slave");
    }
  }

  std::uint8_t get_address() {
    return address;
  }
private:
  std::uint8_t bus;
  std::uint8_t address;
  int fd;
};

};
