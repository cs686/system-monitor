<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

class BatteryData {
    
    // 电池温度
    temperature!: string;
    // 循环周期
    cycle_count!: number;
    // 充电状态
    state!: number;
    // 电量百分比
    percentage!: number;
    // // 还需多久充满
    // time_to_full: u32,
    // // 电池剩余使用时间
    // time_to_empty: u32,
    // 电池健康
    state_of_health!: string;
}


const battery = ref<BatteryData>()

function flushBattery() {
  invoke<BatteryData>("battery_info", {}).then((bat) => {
    battery.value = bat
  })
  return flushBattery;
}

onMounted(async () => {
    // batteryChart.value.setOption(batteryOption);
    setInterval(flushBattery(), 30 * 1000);
})

</script>

<template>
  <div class="card">
    电量百分比是<p>{{ battery?.percentage }}</p>
    循环<p>{{ battery?.cycle_count }}</p>
    健康状况<p>{{ battery?.state_of_health }}</p>
  </div>

</template>
