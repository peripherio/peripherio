use device::category::Category;
use device::io::Interface;

trait Driver {
    type Intf: Interface;

    fn name(&self) -> String;
    fn category(&self) -> Category;
    fn interface(&self) -> Self::Intf;
}
