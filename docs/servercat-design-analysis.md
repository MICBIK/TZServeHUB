# ServerCat 设计分析报告

## 概述

基于 ServerCat iOS 应用的官方截图和 App Store 页面分析，本文档详细记录其界面设计的核心要素，为 ServerHUB 的 UI 改进提供参考。

---

## 1. 圆环配色方案

### 1.1 核心指标圆环

从截图分析，ServerCat 使用**三色圆环系统**展示关键系统指标：

**CPU 圆环**：
- 主色：**蓝色系** (推测 #007AFF 或类似 iOS 系统蓝)
- 用途：CPU 使用率百分比
- 特点：支持多核心独立显示，每个核心一个小圆环

**内存圆环**：
- 主色：**绿色系** (推测 #34C759 或类似 iOS 系统绿)
- 用途：内存使用率百分比
- 显示：已用内存 / 总内存

**磁盘圆环**：
- 主色：**橙色/黄色系** (推测 #FF9500 或 #FFCC00)
- 用途：磁盘空间使用率
- 显示：已用空间 / 总空间

### 1.2 圆环视觉特性

- **渐变效果**：圆环可能使用轻微渐变，从深色到浅色
- **背景轨道**：灰色半透明背景轨道 (约 10-15% 不透明度)
- **线宽**：圆环线宽约 8-12px (相对于圆环直径)
- **端点样式**：圆角端点 (rounded cap)
- **动画**：平滑的进度动画，使用缓动函数

### 1.3 配色系统建议

```css
/* CPU - 蓝色系 */
--cpu-primary: #007AFF;
--cpu-gradient-start: #007AFF;
--cpu-gradient-end: #5AC8FA;
--cpu-background: rgba(0, 122, 255, 0.1);

/* 内存 - 绿色系 */
--memory-primary: #34C759;
--memory-gradient-start: #34C759;
--memory-gradient-end: #30D158;
--memory-background: rgba(52, 199, 89, 0.1);

/* 磁盘 - 橙色系 */
--disk-primary: #FF9500;
--disk-gradient-start: #FF9500;
--disk-gradient-end: #FFCC00;
--disk-background: rgba(255, 149, 0, 0.1);

/* 网络 - 紫色系 (扩展) */
--network-primary: #AF52DE;
--network-gradient-start: #AF52DE;
--network-gradient-end: #BF5AF2;
--network-background: rgba(175, 82, 222, 0.1);
```

---

## 2. 服务器卡片布局

### 2.1 卡片尺寸与间距

**单卡片尺寸** (iPhone 屏幕参考)：
- 宽度：屏幕宽度 - 32px (左右各 16px margin)
- 高度：约 180-220px (根据内容动态调整)
- 圆角：12-16px (iOS 标准圆角)

**网格布局** (多服务器视图)：
- 列数：iPhone 竖屏 1 列，iPad/横屏 2-3 列
- 卡片间距：16px (垂直和水平)
- 外边距：16px (容器到屏幕边缘)

### 2.2 卡片内部结构

**顶部区域** (Header):
- 服务器名称：18-20px，粗体 (SF Pro Display Bold)
- 状态指示器：8px 圆点 (绿色=在线，红色=离线，灰色=未知)
- IP 地址/主机名：14px，次要文字颜色 (#8E8E93)
- 高度：约 44-50px

**中部区域** (Metrics):
- 三个圆环横向排列
- 圆环直径：60-80px
- 圆环间距：12-16px
- 每个圆环下方显示：
  - 指标名称 (12px，次要颜色)
  - 数值 (16-18px，粗体，主色)

**底部区域** (Footer):
- 次要指标：网络速度、进程数、运行时间
- 字体：12-14px
- 布局：横向排列，等宽分布
- 高度：约 30-40px

### 2.3 卡片视觉样式

```css
.server-card {
  background: rgba(255, 255, 255, 0.95); /* 浅色模式 */
  background: rgba(28, 28, 30, 0.95);    /* 深色模式 */
  border-radius: 16px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  backdrop-filter: blur(20px);
  padding: 16px;
}

.server-card:hover {
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
  transform: translateY(-2px);
  transition: all 0.2s ease;
}
```

---

## 3. 关键指标展示方式

### 3.1 数值格式化

**百分比指标** (CPU/内存/磁盘):
- 格式：`85%` 或 `85.2%` (根据精度需求)
- 颜色：
  - 0-60%: 正常色 (绿色/蓝色)
  - 60-80%: 警告色 (橙色 #FF9500)
  - 80-100%: 危险色 (红色 #FF3B30)

**容量指标** (内存/磁盘):
- 格式：`8.2 GB / 16 GB` 或 `450 GB / 1 TB`
- 单位自动换算：B → KB → MB → GB → TB
- 保留 1-2 位小数

**速率指标** (网络):
- 上传：`↑ 1.2 MB/s`
- 下载：`↓ 5.8 MB/s`
- 使用箭头图标区分方向

**时间指标** (运行时间):
- 格式：`15d 8h` 或 `2h 35m`
- 超过 24 小时显示天数

### 3.2 图标系统

ServerCat 使用 **SF Symbols** (iOS 系统图标库)：

- CPU: `cpu` 或 `gauge`
- 内存: `memorychip`
- 磁盘: `internaldrive`
- 网络: `network` 或 `antenna.radiowaves.left.and.right`
- 进程: `list.bullet`
- 温度: `thermometer`
- 电源: `bolt.fill`

**图标尺寸**：
- 主要图标：20-24px
- 次要图标：16-18px
- 状态图标：12-14px

### 3.3 实时更新动画

- **数值变化**：平滑过渡，0.3s ease-out
- **圆环进度**：弹性动画，0.5s ease-in-out
- **状态切换**：淡入淡出，0.2s
- **刷新指示器**：顶部下拉刷新 + 微妙的脉冲动画

---

## 4. 整体视觉风格

### 4.1 色彩模式

**浅色模式** (Light Mode):
- 背景：#F2F2F7 (iOS 系统灰)
- 卡片背景：#FFFFFF (白色，95% 不透明度)
- 主文字：#000000
- 次要文字：#8E8E93
- 分割线：rgba(0, 0, 0, 0.1)

**深色模式** (Dark Mode):
- 背景：#000000 (纯黑) 或 #1C1C1E (深灰)
- 卡片背景：#2C2C2E (深灰，95% 不透明度)
- 主文字：#FFFFFF
- 次要文字：#8E8E93
- 分割线：rgba(255, 255, 255, 0.1)

### 4.2 圆角与阴影

**圆角规范**：
- 大卡片：16px
- 中等组件：12px
- 小按钮：8px
- 圆环/头像：50% (完全圆形)

**阴影层次**：
```css
/* 一级阴影 (卡片) */
box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);

/* 二级阴影 (悬浮) */
box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);

/* 三级阴影 (模态框) */
box-shadow: 0 8px 32px rgba(0, 0, 0, 0.16);
```

### 4.3 透明度与毛玻璃

**毛玻璃效果** (Frosted Glass):
```css
backdrop-filter: blur(20px) saturate(180%);
-webkit-backdrop-filter: blur(20px) saturate(180%);
background: rgba(255, 255, 255, 0.72); /* 浅色模式 */
background: rgba(28, 28, 30, 0.72);    /* 深色模式 */
```

**透明度层级**：
- 完全不透明：1.0 (主要内容)
- 高不透明度：0.95 (卡片背景)
- 中等不透明度：0.72 (毛玻璃)
- 低不透明度：0.3-0.5 (次要元素)
- 极低不透明度：0.1-0.2 (分割线、背景轨道)

### 4.4 字体系统

**iOS 原生字体栈**：
```css
font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'SF Pro Text', 'Helvetica Neue', sans-serif;
```

**字体大小规范**：
- 大标题：28-34px (粗体)
- 标题：20-24px (粗体)
- 正文：16-18px (常规)
- 次要文字：14px (常规)
- 说明文字：12px (常规)
- 数值：18-24px (粗体或 Medium)

**字重**：
- 粗体：700 (SF Pro Display Bold)
- 中等：500-600 (SF Pro Display Medium/Semibold)
- 常规：400 (SF Pro Text Regular)

---

## 5. 多服务器网格布局

### 5.1 响应式布局

**断点设计**：
```css
/* 手机竖屏 */
@media (max-width: 767px) {
  grid-template-columns: 1fr;
  gap: 16px;
  padding: 16px;
}

/* 平板/手机横屏 */
@media (min-width: 768px) and (max-width: 1023px) {
  grid-template-columns: repeat(2, 1fr);
  gap: 20px;
  padding: 20px;
}

/* 桌面 */
@media (min-width: 1024px) {
  grid-template-columns: repeat(3, 1fr);
  gap: 24px;
  padding: 24px;
}

/* 大屏幕 */
@media (min-width: 1440px) {
  grid-template-columns: repeat(4, 1fr);
  gap: 24px;
  padding: 32px;
}
```

### 5.2 网格容器

```css
.server-grid {
  display: grid;
  grid-auto-rows: minmax(200px, auto);
  align-items: start;
  width: 100%;
  max-width: 1920px;
  margin: 0 auto;
}
```

### 5.3 卡片排序与过滤

**排序选项**：
- 按名称 (A-Z)
- 按状态 (在线优先)
- 按 CPU 使用率 (高到低)
- 按内存使用率 (高到低)
- 自定义顺序 (拖拽排序)

**过滤选项**：
- 全部服务器
- 仅在线
- 仅离线
- 按标签分组

### 5.4 空状态设计

**无服务器时**：
- 居中显示大图标 (服务器图标，64px)
- 主文案："还没有添加服务器"
- 次要文案："点击右上角 + 按钮添加第一台服务器"
- CTA 按钮："添加服务器"

---

## 6. 交互细节

### 6.1 手势操作

- **点击卡片**：进入服务器详情页
- **长按卡片**：显示快捷菜单 (编辑/删除/刷新)
- **下拉刷新**：刷新所有服务器数据
- **左滑卡片**：显示删除按钮 (iOS 风格)

### 6.2 加载状态

**骨架屏** (Skeleton):
- 卡片轮廓保持
- 圆环显示灰色占位符
- 文字显示灰色条状占位符
- 脉冲动画 (1.5s 循环)

**刷新指示器**：
- 顶部下拉：iOS 原生刷新控件
- 卡片内刷新：小型旋转图标 (16px)

### 6.3 错误状态

**连接失败**：
- 圆环显示为灰色
- 数值显示为 "—"
- 卡片底部显示错误提示："连接失败"
- 重试按钮

**数据过期**：
- 圆环半透明显示
- 显示最后更新时间："5 分钟前"
- 自动重试机制

---

## 7. 性能优化建议

### 7.1 渲染优化

- 使用虚拟滚动 (超过 20 个服务器时)
- 圆环使用 Canvas 或 SVG (根据性能选择)
- 数值变化使用 CSS transform (避免重排)
- 图片懒加载

### 7.2 动画性能

```css
/* 使用 GPU 加速 */
.server-card {
  will-change: transform;
  transform: translateZ(0);
}

/* 避免触发重排的属性 */
.metric-value {
  transition: opacity 0.3s, transform 0.3s;
  /* 避免使用 width, height, top, left */
}
```

---

## 8. 实现优先级

### Phase 1: 核心视觉 (Week 1)
1. ✅ 三色圆环系统 (CPU/内存/磁盘)
2. ✅ 服务器卡片基础布局
3. ✅ 响应式网格系统
4. ✅ 浅色/深色模式切换

### Phase 2: 交互增强 (Week 2)
1. 圆环动画与数值过渡
2. 卡片悬浮效果
3. 加载与错误状态
4. 下拉刷新

### Phase 3: 高级功能 (Week 3)
1. 拖拽排序
2. 快捷菜单
3. 虚拟滚动 (大量服务器)
4. 毛玻璃效果优化

---

## 9. 关键差异点 (ServerCat vs ServerHUB)

| 特性 | ServerCat (iOS) | ServerHUB (Desktop) |
|------|----------------|---------------------|
| 平台 | iOS 原生 | Tauri 桌面应用 |
| 布局 | 单列滚动 | 多列网格 |
| 交互 | 触摸手势 | 鼠标悬浮 + 点击 |
| 字体 | SF Pro | 跨平台字体栈 |
| 图标 | SF Symbols | Lucide React |
| 动画 | UIKit 原生 | CSS + Framer Motion |
| 毛玻璃 | 原生 API | CSS backdrop-filter |

---

## 10. 技术实现参考

### 10.1 圆环组件 (React)

```typescript
// 使用 Recharts 的 RadialBarChart
import { RadialBarChart, RadialBar } from 'recharts';

const ActivityRing = ({ value, color, label }) => (
  <RadialBarChart
    width={80}
    height={80}
    data={[{ value, fill: color }]}
    startAngle={90}
    endAngle={-270}
  >
    <RadialBar
      dataKey="value"
      cornerRadius={10}
      background={{ fill: `${color}1A` }}
    />
  </RadialBarChart>
);
```

### 10.2 网格布局 (Tailwind CSS)

```tsx
<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 md:gap-6 p-4 md:p-6">
  {servers.map(server => (
    <ServerCard key={server.id} server={server} />
  ))}
</div>
```

### 10.3 毛玻璃效果 (Tailwind)

```tsx
<div className="bg-white/95 dark:bg-gray-900/95 backdrop-blur-xl rounded-2xl shadow-lg">
  {/* 卡片内容 */}
</div>
```

---

## 参考资源

- **App Store**: https://apps.apple.com/app/servercat/id1501532023
- **官网**: https://servercat.app
- **截图路径**: `/Users/baihaibin/Documents/WorkSpares/ServerHUB/docs/servercat-screenshots/`
- **iOS 设计规范**: https://developer.apple.com/design/human-interface-guidelines/
- **SF Symbols**: https://developer.apple.com/sf-symbols/

---

**分析完成时间**: 2026-03-19
**分析者**: Research Agent
**版本**: v1.0
