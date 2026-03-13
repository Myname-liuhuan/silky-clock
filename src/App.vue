<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";

const now = ref(new Date());
let timer: number | null = null;

const time = computed(() => {
  const h = String(now.value.getHours()).padStart(2, "0");
  const m = String(now.value.getMinutes()).padStart(2, "0");
  const s = String(now.value.getSeconds()).padStart(2, "0");
  return { h, m, s };
});

const dateInfo = computed(() => {
  const days = ["星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"];
  const year = now.value.getFullYear();
  const month = now.value.getMonth() + 1;
  const date = now.value.getDate();
  const day = days[now.value.getDay()];
  return { year, month, date, day };
});

const secondProgress = computed(() => now.value.getSeconds() + now.value.getMilliseconds() / 1000);

function updateTime() {
  now.value = new Date();
}

onMounted(() => {
  updateTime();
  timer = window.setInterval(updateTime, 50); // 更新频率提高以实现平滑动画
});

onUnmounted(() => {
  if (timer) {
    clearInterval(timer);
  }
});
</script>

<template>
  <div class="widget" data-tauri-drag-region>
    <!-- 背景光晕 -->
    <div class="glow-orb"></div>

    <!-- 主内容 -->
    <div class="clock-content" data-tauri-drag-region>
      <!-- 时间 -->
      <div class="time-display" data-tauri-drag-region>
        <span class="time-segment">{{ time.h }}</span>
        <span class="colon">:</span>
        <span class="time-segment">{{ time.m }}</span>
        <span class="colon">:</span>
        <span class="time-segment">{{ time.s }}</span>
      </div>

      <!-- 分隔线 -->
      <div class="divider"></div>

      <!-- 日期信息 -->
      <div class="date-info">
        <span class="weekday">{{ dateInfo.day }}</span>
        <span class="full-date">{{ dateInfo.year }}年{{ dateInfo.month }}月{{ dateInfo.date }}日</span>
      </div>

      <!-- 秒点指示器 -->
      <div class="second-dots">
        <span
          v-for="i in 6"
          :key="i"
          class="dot"
          :class="{ active: secondProgress >= (i - 1) * 10 && secondProgress < i * 10 }"
        ></span>
      </div>
    </div>
  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@300;400;500;600;700&display=swap');

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  width: 100%;
  height: 100%;
  background: transparent;
  font-family: 'Space Grotesk', sans-serif;
}
</style>

<style scoped>
.widget {
  position: relative;
  width: 100%;
  height: 100%;
  border-radius: 20px;
  overflow: hidden;
  background: linear-gradient(
    135deg,
    rgba(255, 255, 255, 0.1) 0%,
    rgba(255, 255, 255, 0.05) 50%,
    rgba(255, 255, 255, 0.02) 100%
  );
  backdrop-filter: blur(24px) saturate(180%);
  -webkit-backdrop-filter: blur(24px) saturate(180%);
  border: 1px solid rgba(255, 255, 255, 0.15);
  box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.2),
    inset 0 -1px 0 rgba(255, 255, 255, 0.05);
  cursor: grab;
  user-select: none;
}

.widget:active {
  cursor: grabbing;
}

/* 背景光晕 */
.glow-orb {
  position: absolute;
  top: -50%;
  left: 50%;
  transform: translateX(-50%);
  width: 120%;
  height: 100%;
  background: radial-gradient(
    ellipse at center,
    rgba(120, 180, 255, 0.15) 0%,
    rgba(100, 150, 255, 0.08) 30%,
    transparent 70%
  );
  animation: breathe 4s ease-in-out infinite;
  pointer-events: none;
}

@keyframes breathe {
  0%, 100% {
    opacity: 0.6;
    transform: translateX(-50%) scale(1);
  }
  50% {
    opacity: 1;
    transform: translateX(-50%) scale(1.1);
  }
}

/* 主内容 */
.clock-content {
  position: relative;
  z-index: 1;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 12px 16px;
  gap: 4px;
}

/* 时间显示 */
.time-display {
  display: flex;
  align-items: center;
  line-height: 1;
}

.time-segment {
  font-size: 42px;
  font-weight: 600;
  color: #ffffff;
  letter-spacing: 1px;
  text-shadow:
    0 0 20px rgba(120, 180, 255, 0.5),
    0 2px 4px rgba(0, 0, 0, 0.3);
  min-width: 58px;
  text-align: center;
}

.colon {
  font-size: 42px;
  font-weight: 300;
  color: rgba(255, 255, 255, 0.7);
  margin: 0 2px;
  animation: blink 1s ease-in-out infinite;
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

/* 分隔线 */
.divider {
  width: 60%;
  height: 1px;
  background: linear-gradient(
    90deg,
    transparent 0%,
    rgba(255, 255, 255, 0.3) 20%,
    rgba(120, 180, 255, 0.5) 50%,
    rgba(255, 255, 255, 0.3) 80%,
    transparent 100%
  );
  margin: 2px 0;
}

/* 日期信息 */
.date-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1px;
}

.weekday {
  font-size: 12px;
  font-weight: 500;
  color: rgba(120, 180, 255, 0.9);
  letter-spacing: 2px;
  text-transform: uppercase;
}

.full-date {
  font-size: 11px;
  font-weight: 300;
  color: rgba(255, 255, 255, 0.6);
  letter-spacing: 1px;
}

/* 秒点指示器 */
.second-dots {
  display: flex;
  gap: 4px;
  margin-top: 4px;
}

.dot {
  width: 4px;
  height: 4px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.2);
  transition: all 0.3s ease;
}

.dot.active {
  background: rgba(120, 180, 255, 0.9);
  box-shadow: 0 0 8px rgba(120, 180, 255, 0.6);
  transform: scale(1.2);
}
</style>
