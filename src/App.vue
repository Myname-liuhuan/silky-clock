<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";

const time = ref("00:00:00");
let timer: number | null = null;

function updateTime() {
  const now = new Date();
  time.value = now.toLocaleTimeString("zh-CN", { hour12: false });
}

onMounted(() => {
  updateTime();
  timer = window.setInterval(updateTime, 1000);
});

onUnmounted(() => {
  if (timer) {
    clearInterval(timer);
  }
});
</script>

<template>
  <div class="clock-container" data-tauri-drag-region>
    <span class="time" data-tauri-drag-region>{{ time }}</span>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  width: 100%;
  height: 100%;
  background: transparent;
}
</style>

<style scoped>
.clock-container {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(30, 30, 30, 0.75);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 16px;
  cursor: grab;
  user-select: none;
}

.clock-container:active {
  cursor: grabbing;
}

.time {
  font-family: "SF Mono", "Menlo", "Monaco", "Consolas", monospace;
  font-size: 48px;
  font-weight: 600;
  color: #ffffff;
  letter-spacing: 2px;
  text-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}
</style>
