<template>
  <div class="message-header">
    <template v-if="!isSelf">
      <div class="sender">{{ sender }}</div>
      <div class="time">{{ time }}</div>
    </template>
    <template v-else>
      <div class="time">{{ time }}</div>
      <div class="sender">{{ sender }}</div>
    </template>
  </div>
</template>

<script>
export default {
  name: 'MessageHeader',
  props: {
    sender: {
      type: String,
      required: true,
    },
    time: {
      type: String,
      required: true,
    },
    isSelf: {
      type: Boolean,
      default: false,
    },
  },
};
</script>

<style scoped>
.message-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 6px;
  padding: 0 2px;
  height: 20px; /* 固定高度确保一致性 */
  margin-top: 8px; /* 调整消息头部位置，与头像对齐 */
}

.sender {
  font-weight: 600;
  font-size: 0.85em;
  color: rgb(168, 168, 168);
  user-select: none;
}

.time {
  font-size: 0.7em;
  color: rgb(170, 170, 170);
  margin-left: 8px;
  margin-right: 8px;
  user-select: none;
}
</style>
