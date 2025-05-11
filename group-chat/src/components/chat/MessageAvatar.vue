<template>
  <div :class="['avatar', isSelf ? 'self' : '']">
    {{ sender.charAt(0) }}
  </div>
</template>

<script>
export default {
  name: 'MessageAvatar',
  props: {
    sender: {
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
.avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background-color: #a5d6a7;
  color: #1b5e20;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  margin: 0 8px;
  font-size: 16px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  user-select: none;
  align-self: flex-start;
  line-height: 1;
  text-align: center;
  padding-bottom: 2px; /* 微调垂直位置 */
}

.avatar.self {
  background: linear-gradient(135deg, #4caf50 0%, #2e7d32 100%);
  color: white;
}
</style>
