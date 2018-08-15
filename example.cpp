#include <iostream>
#include <chrono>
#include <thread>
#include <iomanip>
#include <utility>

#include "mpu6050.hpp"

int main() {
  using namespace std::literals::chrono_literals;

  auto addr = rami::driver::mpu6050::detect(1);
  if(!addr) {
    std::cout << "not found" << std::endl;
    return -1;
  }
  std::cout << "Addr: " << std::hex << static_cast<int>(*addr) << std::dec << std::endl;
  auto device = rami::driver::mpu6050(1, *addr);
  std::cout << device.get_temp() << std::endl;

  while(true) {
    const auto v = device.get_accel_data();
    if(!v) {
      std::cerr << "Failed to get accel data" << std::endl;
    }
    double x,y,z;
    std::tie(x, y, z) = *v;
    std::cout << "x:" << x << std::endl
              << "y:" << y << std::endl
              << "z:" << z << std::endl;
    std::this_thread::sleep_for(1s);
  }
}
