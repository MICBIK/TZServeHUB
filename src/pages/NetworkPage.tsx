import React from 'react';

const NetworkPage: React.FC = () => {
  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6">网络监控</h1>
      {/* TODO: interface traffic charts + latency history */}
      <p className="text-gray-400">网卡流量 + 延迟历史 - 待实现</p>
    </div>
  );
};

export default NetworkPage;
