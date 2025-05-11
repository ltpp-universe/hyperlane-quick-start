<template>
  <div
    class="virtual-list-container"
    ref="container"
    @scroll="onScroll"
    :style="{ height: containerHeight + 'px' }"
  >
    <div
      class="virtual-list-phantom"
      :style="{ height: totalHeight + 'px' }"
    ></div>
    <div
      class="virtual-list-content"
      :style="{ transform: `translateY(${offsetY}px)` }"
    >
      <slot
        v-for="item in visibleItems"
        :item="item"
        :index="item._virtualIndex"
        :key="item._virtualIndex"
      ></slot>
    </div>
  </div>
</template>

<script>
export default {
  name: 'VirtualList',
  props: {
    // 列表数据
    items: {
      type: Array,
      required: true,
    },
    // 容器高度
    containerHeight: {
      type: Number,
      default: 500,
    },
    // 每项的预估高度
    estimatedItemHeight: {
      type: Number,
      default: 80,
    },
    // 缓冲区域的项数
    bufferSize: {
      type: Number,
      default: 5,
    },
  },
  data() {
    return {
      // 可视区域的起始索引
      startIndex: 0,
      // 可视区域的结束索引
      endIndex: 0,
      // 可视区域的偏移量
      offsetY: 0,
      // 容器的可视高度
      clientHeight: 0,
      // 滚动位置
      scrollTop: 0,
      // 每项的实际高度缓存
      itemHeights: [],
    };
  },
  computed: {
    // 可见的列表项
    visibleItems() {
      const items = this.items.slice(this.startIndex, this.endIndex + 1);
      return items.map((item, index) => {
        return {
          ...item,
          _virtualIndex: this.startIndex + index,
        };
      });
    },
    // 列表总高度
    totalHeight() {
      if (this.items.length === 0) return 0;

      // 如果有缓存的高度，使用缓存计算总高度
      if (this.itemHeights.length > 0) {
        // 已测量项的总高度
        const measuredHeight = this.itemHeights.reduce(
          (sum, height) => sum + height,
          0
        );
        // 未测量项的估计总高度
        const unmeasuredCount = this.items.length - this.itemHeights.length;
        const unmeasuredHeight = unmeasuredCount * this.estimatedItemHeight;

        return measuredHeight + unmeasuredHeight;
      }

      // 否则使用估计高度
      return this.items.length * this.estimatedItemHeight;
    },
  },
  watch: {
    items: {
      handler() {
        this.updateVisibleItems();
      },
      deep: true,
    },
  },
  mounted() {
    this.clientHeight = this.$refs.container.clientHeight;
    this.updateVisibleItems();

    // 初始化每项的高度估计值
    this.itemHeights = new Array(this.items.length).fill(
      this.estimatedItemHeight
    );

    // 使用ResizeObserver监听容器大小变化
    if (typeof ResizeObserver !== 'undefined') {
      this.resizeObserver = new ResizeObserver(() => {
        this.clientHeight = this.$refs.container.clientHeight;
        this.updateVisibleItems();
      });
      this.resizeObserver.observe(this.$refs.container);
    }

    // 使用MutationObserver监听列表项内容变化
    this.$nextTick(() => {
      this.updateItemHeights();
    });
  },
  beforeUnmount() {
    if (this.resizeObserver) {
      this.resizeObserver.disconnect();
    }
  },
  methods: {
    // 滚动事件处理
    onScroll(e) {
      this.scrollTop = e.target.scrollTop;

      // 计算距离底部的距离
      const distanceToBottom =
        e.target.scrollHeight - this.scrollTop - this.clientHeight;

      // 如果距离底部小于100px，认为用户在底部附近
      // 当用户滚动超过100px时，新消息就不会自动滚动到底部，而是使用提示组件
      const isNearBottom = distanceToBottom < 100;
      // 通知父组件滚动状态变化
      this.$emit('handleScroll', isNearBottom);

      this.updateVisibleItems();
    },

    // 更新可见项 - 优化性能
    updateVisibleItems() {
      if (!this.$refs.container) return;

      this.scrollTop = this.$refs.container.scrollTop;
      this.clientHeight = this.$refs.container.clientHeight;

      // 计算可视区域内的项
      let startIndex = 0;
      let currentHeight = 0;

      // 添加上方100px的渲染偏移量
      const topOffset = 100;
      const scrollTopWithOffset = Math.max(0, this.scrollTop - topOffset);

      // 使用二分查找优化查找第一个可见项的过程
      if (this.items.length > 50) {
        // 对于大列表使用二分查找
        let low = 0;
        let high = this.items.length - 1;

        while (low <= high) {
          const mid = Math.floor((low + high) / 2);
          let height = 0;

          // 计算到mid位置的累计高度
          for (let i = 0; i < mid; i++) {
            height += this.itemHeights[i] || this.estimatedItemHeight;
          }

          if (height < scrollTopWithOffset) {
            if (
              height + (this.itemHeights[mid] || this.estimatedItemHeight) >
              scrollTopWithOffset
            ) {
              startIndex = mid;
              currentHeight = height;
              break;
            }
            low = mid + 1;
          } else {
            high = mid - 1;
          }
        }
      } else {
        // 对于小列表使用线性查找
        for (let i = 0; i < this.items.length; i++) {
          const height = this.itemHeights[i] || this.estimatedItemHeight;
          if (currentHeight + height > scrollTopWithOffset) {
            startIndex = i;
            break;
          }
          currentHeight += height;
        }
      }

      // 计算偏移量
      this.offsetY = currentHeight;

      // 考虑缓冲区，但不超出数组范围
      this.startIndex = Math.max(0, startIndex - this.bufferSize);

      // 计算可视区域能容纳的项数
      let visibleCount = 0;
      let visibleHeight = 0;

      for (let i = startIndex; i < this.items.length; i++) {
        const height = this.itemHeights[i] || this.estimatedItemHeight;
        visibleHeight += height;
        visibleCount++;

        // 增加下方100px的渲染偏移量
        if (visibleHeight > this.clientHeight + 100) {
          // 增加额外空间以防止快速滚动时出现白屏
          break;
        }
      }

      // 加上缓冲区
      this.endIndex = Math.min(
        this.items.length - 1,
        startIndex + visibleCount + this.bufferSize
      );

      // 更新项高度
      this.$nextTick(() => {
        this.updateItemHeights();
      });
    },

    // 更新项高度缓存
    updateItemHeights() {
      const nodes = this.$el.querySelectorAll('.virtual-list-content > *');

      Array.from(nodes).forEach((node, index) => {
        const realIndex = this.startIndex + index;
        const height = node.offsetHeight;

        // 只有当高度变化时才更新
        if (height && this.itemHeights[realIndex] !== height) {
          this.itemHeights[realIndex] = height;
        }
      });
    },

    // 滚动到指定索引
    scrollToIndex(index) {
      if (index < 0 || index >= this.items.length) return;

      let position = 0;
      for (let i = 0; i < index; i++) {
        position += this.itemHeights[i] || this.estimatedItemHeight;
      }

      this.$refs.container.scrollTop = position;
    },

    // 滚动到底部
    scrollToBottom() {
      if (this.$refs.container) {
        this.$refs.container.scrollTop = this.$refs.container.scrollHeight;
      }
    },
  },
};
</script>

<style scoped>
.virtual-list-container {
  overflow-y: auto;
  position: relative;
  scrollbar-width: thin;
}

.virtual-list-phantom {
  position: absolute;
  left: 0;
  top: 0;
  right: 0;
  z-index: -1;
}

.virtual-list-content {
  position: absolute;
  left: 0;
  right: 0;
  top: 0;
  min-height: 100%;
}
</style>
