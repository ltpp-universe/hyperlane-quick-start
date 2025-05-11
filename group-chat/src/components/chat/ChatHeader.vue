<template>
  <div class="chat-header">
    <h2>
      <i class="header-icon">{{ icon }}</i> {{ title }}
    </h2>
  </div>
</template>

<script>
export default {
  name: 'ChatHeader',
  props: {
    title: {
      type: String,
      required: true,
    },
    icon: {
      type: String,
      default: '💬',
    },
  },
};
</script>

<style scoped>
.chat-header {
  background: linear-gradient(135deg, #4caf50 0%, #2e7d32 100%);
  color: white;
  user-select: none;
  padding: 4px 16px;
  text-align: center;
  position: sticky;
  top: 0;
  z-index: 10;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.chat-header h2 {
  font-size: 1.2rem;
  margin: 0;
}

.header-icon {
  font-style: normal;
  margin-right: 6px;
  font-size: 1em;
}
</style>
