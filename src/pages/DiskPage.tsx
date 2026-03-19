import React from 'react';

const DiskPage: React.FC = () => {
  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6">磁盘监控</h1>
      {/* TODO: disk I/O speed charts + filesystem usage */}
      <p className="text-gray-400">磁盘 I/O 速度 + 空间使用 - 待实现</p>
    </div>
  );
};

export default DiskPage;
