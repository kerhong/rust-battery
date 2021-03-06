use std::fmt;
use std::time::Duration;

use crate::{State, Technology};
use crate::platform::traits::BatteryDevice;
use crate::platform::Device;

/// Battery instant information representation.
///
/// Consequent calls of the same method will return the same value.
pub struct Battery(Device);

impl Battery {

    /// The amount of energy left in the battery expressed as a percentage between `0.0` and `100.0`.
    pub fn percentage(&self) -> f32 {
        self.0.percentage()
    }

    /// Amount of energy (measured in `mWh`) currently available in the battery.
    pub fn energy(&self) -> u32 {
        self.0.energy()
    }

    /// Amount of energy (measured in `mWh`) in the battery when it's considered full.
    pub fn energy_full(&self) -> u32 {
        self.0.energy_full()
    }

    /// Amount of energy (measured in `mWh`) the battery is designed to hold when it's considered full.
    pub fn energy_full_design(&self) -> u32 {
        self.0.energy_full_design()
    }

    /// Amount of energy being drained from the battery, measured in `mW`.
    pub fn energy_rate(&self) -> u32 {
        self.0.energy_rate()
    }

    /// Gets a battery voltage (in `mV`).
    pub fn voltage(&self) -> u32 {
        self.0.voltage()
    }

    /// Gets battery capacity in `0.0..100.0` percents range.
    pub fn capacity(&self) -> f32 {
        self.0.capacity()
    }

    /// Gets battery current state.
    ///
    /// See [State](enum.State.html) enum for possible values.
    pub fn state(&self) -> State {
        self.0.state()
    }

    /// Gets battery technology.
    ///
    /// See [Technology](enum.Technology.html) enum for possible values.
    ///
    /// ## Compatibility
    ///
    /// * For MacOS this method always returns `Technology::Unknown`. This behavior might change later.
    pub fn technology(&self) -> Technology {
        self.0.technology()
    }

    /// Gets battery temperature in Celsius degrees.
    ///
    /// ## Compatibility
    ///
    /// * For FreeBSD this method always returns `None`. This behavior might change later.
    pub fn temperature(&self) -> Option<f32> {
        self.0.temperature()
    }

    /// Number of charge/discharge cycles
    ///
    /// ## Compatibility
    ///
    /// * For FreeBSD this method always returns `None`. This behavior might change later.
    pub fn cycle_count(&self) -> Option<u32> {
        self.0.cycle_count()
    }

    /// Gets battery vendor.
    pub fn vendor(&self) -> Option<&str> {
        self.0.vendor()
    }

    /// Gets battery model.
    pub fn model(&self) -> Option<&str> {
        self.0.model()
    }

    /// Gets battery serial number.
    pub fn serial_number(&self) -> Option<&str> {
        self.0.serial_number()
    }

    /// Gets a remaining time till full battery.
    ///
    /// This is an instant value and may different vastly from call to call.
    /// Any aggregation should be made by caller.
    ///
    /// If battery is not charging at the moment, this method will return `None`.
    pub fn time_to_full(&self) -> Option<Duration> {
        self.0.time_to_full()
    }

    /// Gets a remaining time till empty battery.
    ///
    /// This is an instant value and may different vastly from call to call.
    /// Any aggregation should be made by caller.
    ///
    /// If battery is not discharging at the moment, this method will return `None`.
    pub fn time_to_empty(&self) -> Option<Duration> {
        self.0.time_to_empty()
    }

    #[allow(dead_code)]
    pub(crate) fn get_ref(&self) -> &Device {
        &self.0
    }

    pub(crate) fn get_mut_ref(&mut self) -> &mut Device {
        &mut self.0
    }
}

impl fmt::Debug for Battery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Battery")
            // static info
            .field("vendor", &self.vendor())
            .field("model", &self.model())
            .field("serial_number", &self.serial_number())
            .field("technology", &self.technology())

            // common information
            .field("state", &self.state())
            .field("capacity", &self.capacity())
            .field("temperature", &self.temperature())
            .field("percentage", &self.percentage())
            .field("cycle_count", &self.cycle_count())

            // energy stats
            .field("energy", &self.energy())
            .field("energy_full", &self.energy_full())
            .field("energy_full_design", &self.energy_full_design())
            .field("energy_rate", &self.energy_rate())
            .field("voltage", &self.voltage())

            // charge stats
            .field("time_to_full", &self.time_to_full())
            .field("time_to_empty", &self.time_to_empty())

            .finish()
    }
}

impl From<Device> for Battery {
    fn from(inner: Device) -> Self {
        Battery(inner)
    }
}
