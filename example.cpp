#include <iostream>
#include <chrono>
#include <thread>
#include <iomanip>

#include "mpu6050.hpp"
#include "pca9685.hpp"

int main() {
  using namespace std::literals::chrono_literals;

  auto addr = rami::driver::pca9685::detect(1);
  if(addr == 0) {
    std::cout << "not found" << std::endl;
    return -1;
  } else {
    std::cout << "A: " << std::hex << static_cast<int>(addr) << std::dec << std::endl;
    /* auto device = rami::driver::mpu6050(1, addr); */
    /* std::cout << device.get_temp() << std::endl; */
  }
  /* while(true) { */
  /*   auto const [x, y, z] = device.get_accel_data(); */
  /*   std::cout << "x:" << x << std::endl */
  /*             << "y:" << y << std::endl */
  /*             << "z:" << z << std::endl; */
  /*   std::this_thread::sleep_for(1s); */
  /* } */
}
