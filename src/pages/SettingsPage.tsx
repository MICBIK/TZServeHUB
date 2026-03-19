import React from 'react';

const SettingsPage: React.FC = () => {
  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6">设置</h1>
      {/* TODO: server management + polling config + theme + language */}
      <p className="text-gray-400">服务器管理 + 轮询配置 + 主题 + 语言 - 待实现</p>
    </div>
  );
};

export default SettingsPage;
