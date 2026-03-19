import React from 'react';

const ProbePage: React.FC = () => {
  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6">网络探测</h1>
      {/* TODO: ping/TCP/DNS probe history with vantage_point labels */}
      <p className="text-gray-400">Ping / TCP / DNS 探测历史 - 待实现</p>
    </div>
  );
};

export default ProbePage;
