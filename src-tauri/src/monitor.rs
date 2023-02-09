use battery::{Battery, Manager};
#[derive(Debug, serde::Serialize, Default)]
pub struct BatteryData {
    // 电池温度
    temperature: String,
    // 循环周期
    cycle_count: u32,
    // 充电状态
    state: i32,
    // 电量百分比
    percentage: f32,
    // // 还需多久充满
    // time_to_full: u32,
    // // 电池剩余使用时间
    // time_to_empty: u32,
    // 电池健康
    state_of_health: String,
}

impl BatteryData {
    pub fn new(battery: &Battery) -> Self {
        BatteryData {
            temperature: format!("{:.2}℃", battery.temperature().unwrap().value - 273.15),
            cycle_count: battery.cycle_count().unwrap(),
            state: match battery.state() {
                battery::State::Full => 1,
                battery::State::Charging => 2,
                battery::State::Discharging => 3,
                battery::State::Empty => 4,
                _ => -1,
            },
            percentage: battery.state_of_charge().value * 100.,
            state_of_health: format!("{:.2}%", battery.state_of_health().value * 100.),
        }
    }
}

#[tauri::command]
pub fn battery_info() -> BatteryData {
    let battery_manager = Manager::new().unwrap();
    let mut batteries = vec![];

    for (_, battery) in battery_manager.batteries().unwrap().enumerate() {
        batteries.push(BatteryData::new(&battery.unwrap()));
    }
    if batteries.len() == 0 {
        return BatteryData::default();
    } else {
        return batteries.pop().unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sysinfo_fn() {}
}
