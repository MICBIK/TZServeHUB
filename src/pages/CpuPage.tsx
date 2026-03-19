import React from 'react';

const CpuPage: React.FC = () => {
  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6">CPU 监控</h1>
      {/* TODO: total CPU chart + per-core charts */}
      <p className="text-gray-400">CPU 总使用率 + 每核心图表 - 待实现</p>
    </div>
  );
};

export default CpuPage;
