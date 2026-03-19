import React from 'react';
import { NavLink } from 'react-router-dom';

const navItems = [
  { path: '/', label: 'Dashboard', icon: '📊' },
  { path: '/cpu', label: 'CPU', icon: '🔲' },
  { path: '/network', label: '网络', icon: '🌐' },
  { path: '/disk', label: '磁盘', icon: '💾' },
  { path: '/probes', label: '探测', icon: '📡' },
  { path: '/alerts', label: '告警', icon: '🔔' },
  { path: '/settings', label: '设置', icon: '⚙️' },
];

const Sidebar: React.FC = () => {
  return (
    <aside className="w-56 bg-gray-900 border-r border-gray-800 flex flex-col h-full">
      <div className="p-4 border-b border-gray-800">
        <h1 className="text-lg font-bold text-white">ServerHUB</h1>
        <p className="text-xs text-gray-500 mt-1">Server Monitoring</p>
      </div>
      <nav className="flex-1 py-2">
        {navItems.map((item) => (
          <NavLink
            key={item.path}
            to={item.path}
            className={({ isActive }) =>
              `flex items-center gap-3 px-4 py-2.5 text-sm transition-colors ${
                isActive
                  ? 'bg-gray-800 text-white border-r-2 border-blue-500'
                  : 'text-gray-400 hover:text-white hover:bg-gray-800/50'
              }`
            }
          >
            <span>{item.icon}</span>
            <span>{item.label}</span>
          </NavLink>
        ))}
      </nav>
    </aside>
  );
};

export default Sidebar;
