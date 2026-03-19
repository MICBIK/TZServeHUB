import React from 'react';

const AlertPage: React.FC = () => {
  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6">告警日志</h1>
      {/* TODO: alert event list + rule management */}
      <p className="text-gray-400">告警事件列表 + 规则管理 - 待实现</p>
    </div>
  );
};

export default AlertPage;
