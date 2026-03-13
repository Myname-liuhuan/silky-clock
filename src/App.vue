<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface WidgetState {
  is_draggable: boolean;
  is_click_through: boolean;
}

const now = ref(new Date());
const isDraggable = ref(true);
const isClickThrough = ref(false);
const showToast = ref(false);
const toastMessage = ref("");
const isHovering = ref(false);
let timer: number | null = null;
let hoverTimeout: number | null = null;

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

async function fetchState() {
  try {
    const state = await invoke<WidgetState>("get_widget_state");
    isDraggable.value = state.is_draggable;
    isClickThrough.value = state.is_click_through;
  } catch (e) {
    console.error("Failed to fetch state:", e);
  }
}

async function toggleDragMode() {
  try {
    const state = await invoke<WidgetState>("toggle_drag_mode");
    isDraggable.value = state.is_draggable;
    displayToast(state.is_draggable ? "已启用拖动" : "已锁定位置");
  } catch (e) {
    console.error("Failed to toggle drag mode:", e);
  }
}

async function toggleClickThrough() {
  try {
    const state = await invoke<WidgetState>("toggle_click_through");
    isClickThrough.value = state.is_click_through;
    displayToast(state.is_click_through ? "已启用点击穿透" : "已关闭点击穿透");
  } catch (e) {
    console.error("Failed to toggle click through:", e);
  }
}

function displayToast(message: string) {
  toastMessage.value = message;
  showToast.value = true;
  setTimeout(() => {
    showToast.value = false;
  }, 1500);
}

function handleMouseEnter() {
  if (hoverTimeout) clearTimeout(hoverTimeout);
  hoverTimeout = window.setTimeout(() => {
    isHovering.value = true;
  }, 200);
}

function handleMouseLeave() {
  if (hoverTimeout) clearTimeout(hoverTimeout);
  isHovering.value = false;
}

async function handleContextMenu(e: MouseEvent) {
  e.preventDefault();
  try {
    await invoke("show_context_menu");
  } catch (err) {
    console.error("Failed to show context menu:", err);
  }
}

onMounted(async () => {
  updateTime();
  timer = window.setInterval(updateTime, 50);

  // Fetch initial state
  await fetchState();

  // Listen for state changes from backend (context menu)
  await listen("widget-state-changed", async () => {
    await fetchState();
  });

  // Listen for toast events from backend
  await listen<string>("show-toast", (event) => {
    displayToast(event.payload);
  });
});

onUnmounted(() => {
  if (timer) {
    clearInterval(timer);
  }
  if (hoverTimeout) {
    clearTimeout(hoverTimeout);
  }
});
</script>

<template>
  <div
    class="widget"
    :class="{
      fixed: !isDraggable,
      'click-through': isClickThrough,
    }"
    @mouseenter="handleMouseEnter"
    @mouseleave="handleMouseLeave"
    @contextmenu="handleContextMenu"
  >
    <!-- 背景光晕 -->
    <div class="glow-orb"></div>

    <!-- 悬停工具栏 -->
    <Transition name="toolbar-fade">
      <div v-if="isHovering" class="hover-toolbar">
        <button
          class="toolbar-btn"
          :class="{ active: !isDraggable }"
          @click="toggleDragMode"
          title="锁定/解锁位置"
        >
          <svg v-if="isDraggable" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
          </svg>
          <svg v-else viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z"/>
          </svg>
        </button>
        <button
          class="toolbar-btn"
          :class="{ active: isClickThrough }"
          @click="toggleClickThrough"
          title="点击穿透"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10" stroke-dasharray="4 2" />
            <circle cx="12" cy="12" r="3" />
          </svg>
        </button>
      </div>
    </Transition>

    <!-- 主内容 -->
    <div
      class="clock-content"
      :data-tauri-drag-region="isDraggable ? true : undefined"
    >
      <!-- 时间 -->
      <div
        class="time-display"
        :data-tauri-drag-region="isDraggable ? true : undefined"
      >
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

    <!-- Toast 提示 -->
    <Transition name="toast-fade">
      <div v-if="showToast" class="toast">
        {{ toastMessage }}
      </div>
    </Transition>
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
  /* Disable text selection globally */
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
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
  /* Disable text selection */
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
  transition: border-color 0.3s ease, box-shadow 0.3s ease;
}

.widget:active {
  cursor: grabbing;
}

.widget.fixed {
  border-color: rgba(255, 150, 120, 0.3);
  box-shadow:
    0 8px 32px rgba(255, 150, 120, 0.15),
    inset 0 1px 0 rgba(255, 255, 255, 0.2),
    inset 0 -1px 0 rgba(255, 255, 255, 0.05);
  cursor: default;
}

.widget.click-through {
  border-color: rgba(180, 120, 255, 0.3);
  box-shadow:
    0 8px 32px rgba(180, 120, 255, 0.15),
    inset 0 1px 0 rgba(255, 255, 255, 0.2),
    inset 0 -1px 0 rgba(255, 255, 255, 0.05);
}

/* 悬停工具栏 */
.hover-toolbar {
  position: absolute;
  top: 8px;
  right: 8px;
  display: flex;
  gap: 4px;
  z-index: 10;
}

.toolbar-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.15);
  border-radius: 8px;
  color: rgba(255, 255, 255, 0.7);
  cursor: pointer;
  transition: all 0.2s ease;
}

.toolbar-btn:hover {
  background: rgba(255, 255, 255, 0.2);
  color: #ffffff;
}

.toolbar-btn.active {
  background: rgba(120, 180, 255, 0.3);
  color: rgba(120, 180, 255, 1);
  border-color: rgba(120, 180, 255, 0.5);
}

.widget.fixed .toolbar-btn.active {
  background: rgba(255, 150, 120, 0.3);
  color: rgba(255, 150, 120, 1);
  border-color: rgba(255, 150, 120, 0.5);
}

.widget.click-through .toolbar-btn.active {
  background: rgba(180, 120, 255, 0.3);
  color: rgba(180, 120, 255, 1);
  border-color: rgba(180, 120, 255, 0.5);
}

.toolbar-btn svg {
  width: 16px;
  height: 16px;
}

.toolbar-fade-enter-active,
.toolbar-fade-leave-active {
  transition: all 0.2s ease;
}

.toolbar-fade-enter-from,
.toolbar-fade-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

/* Toast 提示 */
.toast {
  position: absolute;
  bottom: -30px;
  left: 50%;
  transform: translateX(-50%);
  padding: 6px 12px;
  background: rgba(40, 40, 50, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: rgba(255, 255, 255, 0.9);
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
  z-index: 100;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.toast-fade-enter-active,
.toast-fade-leave-active {
  transition: all 0.3s ease;
}

.toast-fade-enter-from,
.toast-fade-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(10px);
}

/* 背景光晕 */
.glow-orb {
  position: absolute;
  inset: 0;
  background: radial-gradient(
    ellipse at center,
    rgba(120, 180, 255, 0.15) 0%,
    rgba(100, 150, 255, 0.08) 30%,
    transparent 70%
  );
  animation: breathe 4s ease-in-out infinite;
  pointer-events: none;
}

.widget.fixed .glow-orb {
  background: radial-gradient(
    ellipse at center,
    rgba(255, 150, 120, 0.12) 0%,
    rgba(255, 120, 100, 0.06) 30%,
    transparent 70%
  );
}

.widget.click-through .glow-orb {
  background: radial-gradient(
    ellipse at center,
    rgba(180, 120, 255, 0.12) 0%,
    rgba(150, 100, 255, 0.06) 30%,
    transparent 70%
  );
}

@keyframes breathe {
  0%, 100% {
    opacity: 0.6;
  }
  50% {
    opacity: 1;
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

.widget.fixed .time-segment {
  text-shadow:
    0 0 20px rgba(255, 150, 120, 0.4),
    0 2px 4px rgba(0, 0, 0, 0.3);
}

.widget.click-through .time-segment {
  text-shadow:
    0 0 20px rgba(180, 120, 255, 0.4),
    0 2px 4px rgba(0, 0, 0, 0.3);
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

.widget.fixed .divider {
  background: linear-gradient(
    90deg,
    transparent 0%,
    rgba(255, 255, 255, 0.3) 20%,
    rgba(255, 150, 120, 0.5) 50%,
    rgba(255, 255, 255, 0.3) 80%,
    transparent 100%
  );
}

.widget.click-through .divider {
  background: linear-gradient(
    90deg,
    transparent 0%,
    rgba(255, 255, 255, 0.3) 20%,
    rgba(180, 120, 255, 0.5) 50%,
    rgba(255, 255, 255, 0.3) 80%,
    transparent 100%
  );
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

.widget.fixed .weekday {
  color: rgba(255, 150, 120, 0.9);
}

.widget.click-through .weekday {
  color: rgba(180, 120, 255, 0.9);
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

.widget.fixed .dot.active {
  background: rgba(255, 150, 120, 0.9);
  box-shadow: 0 0 8px rgba(255, 150, 120, 0.6);
}

.widget.click-through .dot.active {
  background: rgba(180, 120, 255, 0.9);
  box-shadow: 0 0 8px rgba(180, 120, 255, 0.6);
}
</style>
